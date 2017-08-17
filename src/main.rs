#![feature(no_core)]
#![feature(core_str_ext)]
#![feature(lang_items)]
#![feature(asm)]
#![no_core]
#![no_main]

/* 
 * Stripped down version of the rust core lib https://github.com/gergoerdi/rust-avr-libcore-mini/
 * Arduino library for rust https://github.com/avr-rust/arduino
 * Rust libc https://github.com/rust-lang/rlibc (ORIGINAL)
 */
extern crate core;
extern crate arduino;
extern crate rlibc;

// Required core functions
#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! { loop {} }

// Uses
use arduino::serial;
#[allow(unused_imports)]
use core::ops;
#[allow(unused_imports)]
use core::iter;
#[allow(unused_imports)]
use core::option;
use core::result::Result::{Ok, Err};
use core::str::StrExt;

mod parser;
mod interpreter;

use parser::read_program;
use interpreter::Environment;

// Constants
const CPU_FREQUENCY_HZ: u64 = 16_000_000;
const BAUD: u64 = 9600;
const UBRR: u16 = (CPU_FREQUENCY_HZ / 16 / BAUD - 1) as u16;

#[no_mangle]
pub extern fn main() {

    // Serial to interface with USB so that instructions can be taken
    serial::Serial::new(UBRR)
    .character_size(serial::CharacterSize::EightBits)
    .mode(serial::Mode::Asynchronous)
    .parity(serial::Parity::Disabled)
    .stop_bits(serial::StopBits::OneBit)
    .configure();

    let mut bf_interpreter = Environment::new();

    loop {
        match bf_interpreter.run(read_program()) {
            Ok(_) => {}
            Err(msg) => for &b in msg.as_bytes() {
                    serial::transmit(b);
            }
        }
    }
}
