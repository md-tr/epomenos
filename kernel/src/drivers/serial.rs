use crate::utils::low_lvl::{inb, outb};
use core::num::NonZero;
use paste::paste;
use thiserror_no_std::Error;

#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum Port {
    COM1 = 0x3f8,
}

#[derive(Error, Debug)]
pub enum SerialError {
    #[error("cannot set baud to `{0}`: too large (divisor has to be >1)")]
    BaudValueTooLarge(NonZero<u32>),

    #[error("cannot set baud to `{0}`: too small (divisor has to be <65536)")]
    BaudValueTooSmall(NonZero<u32>),

    #[error("serial loopback test failed")]
    LoopbackTestFailed,
}

#[derive(Clone, Copy, Debug)]
pub struct Serial {
    pub port: Port,
}

macro_rules! serial_fn {
    (
        $pb:vis $fn_name:ident ( $self:ident ) < ( $offset:expr )
            $($prelude:block)?
    ) => {
        $pb fn $fn_name(&$self) -> u8 {
            $($prelude)?

            unsafe {
                inb($self.port as u16 + $offset)
            }
        }
    };

    (
        $pb:vis $fn_name:ident ( $self:ident ) > | $var:ident |
            ( $offset:expr , $val:expr )
    ) => {
        $pb fn $fn_name(&$self, $var: u8) {
            unsafe {
                outb($self.port as u16 + $offset, $val);
            }
        }
    };

    (
        $pb:vis $fn_name:ident ( $self:ident ) > | $var:ident |
            ( $offset:expr ) $prelude:block $(::: $epilog:block)?
    ) => {
        $pb fn $fn_name(&$self, $var: u8) {
            paste! {
                let [<tmp_ $var>]: u8 = $prelude;

                unsafe {
                    outb($self.port as u16 + $offset, [<tmp_ $var>]);
                }

                $($epilog)?
            }
        }
    };

    (
        $pb:vis $fn_name:ident ( $self:ident ) > | $var:ident : $vartype:tt |
            ( $offset:expr ) $prelude:block $(::: $epilog:block)?
    ) => {
        $pb fn $fn_name(&$self, $var: $vartype) {
            paste! {
                let [<tmp_ $var>]: u8 = $prelude;

                unsafe {
                    outb($self.port as u16 + $offset, [<tmp_ $var>]);
                }

                $($epilog)?
            }
        }
    }
}

fn _get_digit(n: u8) -> u8 {
    if n < 10 {
        b'0' + n
    } else {
        b'A' + n - 10
    }
}

impl Serial {
    pub fn new(port: Port) -> Serial {
        Serial {
            port,
        }
    }

    pub fn begin(&self, baud: NonZero<u32>) -> Result<(), SerialError> {
        self.ier_write(0x00);
        self.set_baud(baud)?;
        self.lcr_write(0x03);
        self.fifo_write(0xc7);
        self.mcr_write(0x0b);
        self.mcr_write(0x1e);

        self.transmit_byte(0xe9);

        if self.recieve_byte() != 0xe9 {
            return Err(SerialError::LoopbackTestFailed);
        }

        self.mcr_write(0x0f);

        Ok(())
    }

    pub fn read_char(&self) -> char {
        let mut bytes: [u8; 4] = [0; 4];
        let mut val: u32;

        fn invalid_err() -> ! {
            panic!("invalid character read from serial; exiting now");
        }

        while !self.serial_recieved() {}

        for i in 0..4 {
            bytes[i] = self.recieve_byte();

            if bytes[i] == 0 {
                break;
            }
        }

        let byte_count: usize = bytes.iter().filter(|&&i| i > 0).count();

        let ones: usize = bytes[0].leading_ones() as usize;

        if byte_count > 1 && byte_count == ones {
            val = bytes[0] as u32 & ((1 << (7 - ones)) - 1);

            bytes[1..byte_count].iter()
                .map(|x| if x.leading_ones() != 1 {
                    invalid_err()
                } else {
                    x
                })
                .for_each(|&x| {
                    val <<= 6;
                    val |= x as u32 & 0b00111111;
                });
        } else if ones == 0 {
            val = bytes[0] as u32;
        } else {
            invalid_err();
        }

        match char::from_u32(val) {
            Some(c) => c,
            None => invalid_err()
        }
    }

    pub fn read_byte(&self) -> u8 {
        while !self.serial_recieved() {}

        self.recieve_byte()
    }

    pub fn serial_recieved(&self) -> bool {
        (self.lsr_read() & 0x01) != 0
    }

    pub fn writeln(&self, val: &str) {
        self.write_bytes(val.as_bytes());
        self.write_byte(b'\n');
    }

    pub fn write(&self, val: &str) {
        self.write_bytes(val.as_bytes());
    }

    pub fn write_bytes(&self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(byte.clone());
        }
    }

    pub fn write_char(&self, val: char) {
        let mut bytes: [u8; 4] = [0; 4];

        val.encode_utf8(&mut bytes);

        for byte in bytes {
            self.write_byte(byte);
        }
    }

    pub fn write_uint(&self, val: usize, radix: usize) {
        if !matches!(radix, 2..=34) {
            panic!("bad radix for `write_num`");
        }

        if val < radix {
            self.write_byte(_get_digit(val as u8));
        } else {
            self.write_uint(val / radix, radix);
            self.write_byte(_get_digit((val % radix) as u8));
        }
    }
    
    pub fn write_byte(&self, val: u8) {
        while !self.is_transmit_empty() {}

        self.transmit_byte(val);
    }

    pub fn is_transmit_empty(&self) -> bool {
        (self.lsr_read() & 0x20) != 0
    }
    
    pub fn set_baud(&self, baud: NonZero<u32>) -> Result<NonZero<u16>, SerialError> {
        let divisor = self.calculate_divisor(baud)?;

        self.set_divisor(divisor);

        Ok(divisor)
    }

    pub fn calculate_divisor(&self, baud: NonZero<u32>) -> Result<NonZero<u16>, SerialError> {
        let checkdiv_divisor = u16::try_from(115200u32 / baud.get());

        if let Ok(maybezero_divisor) = checkdiv_divisor {
            let nz_divisor = NonZero::new(maybezero_divisor);

            if let Some(divisor) = nz_divisor {
                Ok(divisor)
            } else {
                Err(SerialError::BaudValueTooLarge(baud))
            }
        } else {
            Err(SerialError::BaudValueTooSmall(baud))
        }
    }

    pub fn set_divisor(&self, divisor: NonZero<u16>) {
        let div = divisor.get();

        self.set_divisor_lsb((div & 0xff) as u8);
        self.set_divisor_msb((div >> 8) as u8);
    }

    serial_fn! { pub recieve_byte(self) < (0) }

    serial_fn! { pub transmit_byte(self) > | val | (0, val) }
    
    serial_fn! {
        pub set_divisor_lsb(self) > | lsb | (0) {
            self.dlab_enable(true);

            lsb
        } ::: {
            self.dlab_enable(false);
        }
    }

    serial_fn! {
        pub set_divisor_msb(self) > | msb | (1) {
            self.dlab_enable(true);

            msb
        } ::: {
            self.dlab_enable(false);
        }
    }

    serial_fn! { pub ier_read(self) < (1) }

    serial_fn! { pub ier_write(self) > | ier | (1, ier) }

    serial_fn! { pub isr_read(self) < (2) }

    serial_fn! { pub fifo_write(self) > | fifo | (2, fifo) }

    serial_fn! { pub lcr_read(self) < (3) }

    serial_fn! { pub lcr_write(self) > | lcr | (3, lcr) }

    serial_fn! {
        pub dlab_enable(self) > | enable: bool | (3) {
            let mut lcr = self.lcr_read();

            if enable {
                lcr |= 0b10000000;
            } else {
                lcr &= 0b01111111;
            }

            lcr
        }
    }

    serial_fn! { pub mcr_read(self) < (4) }

    serial_fn! { pub mcr_write(self) > | mcr | (4, mcr) }

    serial_fn! { pub lsr_read(self) < (5) }

    serial_fn! { pub msr_read(self) < (6) }

    serial_fn! { pub spr_read(self) < (7) }

    serial_fn! { pub spr_write(self) > | spr | (7, spr) }
}