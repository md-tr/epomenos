use core::panic::PanicInfo;
use crate::bootl;
use crate::drivers::serial::{Serial, Port};

pub fn kstart_bootl() {
    crate::kmain();
}

#[panic_handler]
fn panic_impl(_info: &PanicInfo) -> ! {
    let ser = Serial::new(Port::COM1);
    let msg = _info.message().as_str().unwrap_or("<no message supplied>");
    let banner_len = core::cmp::max(msg.len(), 5) + 2;

    ser.write("\n\x1b[31m");

    for _ in 0..banner_len {
        ser.write_byte(b'=');
    }

    ser.write_byte(b'\n');

    ser.writeln(&msg);

    ser.write_byte(b'\n');

    for _ in 0..banner_len {
        ser.write_byte(b'=');
    }

    ser.write("\x1b[0m");

    bootl::hcf();
}