#![no_std]
#![no_main]
#![deny(clippy::alloc_instead_of_core)]

pub mod arch;
pub mod bootl;
pub mod drivers;
pub mod utils;

use core::num::NonZero;

use drivers::serial::{Serial, Port};

pub fn kmain() {
    let ser = Serial::new(Port::COM1);
    let mut data: char;

    ser.begin(NonZero::new(38400).unwrap()).expect("Serial error!");

    ser.writeln("\u{3B5}\u{3C0}\u{3CC}\u{3BC}\u{3B5}\u{3BD}\u{3BF}\u{3C2}");

    loop {
        data = ser.read_char();
        ser.write_uint(data as usize, 16);
        ser.write_byte(b';');
    }
}