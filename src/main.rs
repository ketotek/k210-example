#![allow(warnings)]
#![no_std]
#![no_main]

extern crate panic_halt;

use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_hal::Peripherals;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    // Configure clocks (TODO)
    let clocks = k210_hal::clock::Clocks::new();
    let p = Peripherals::take().unwrap();

    // Configure UART
    let serial = p
        .UARTHS
        .configure((p.pins.pin5, p.pins.pin4), 115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);

    writeln!(stdout, "Hello, Rust!").unwrap();

    loop {
        writeln!(stdout, "Hello again!").unwrap();
    }
}
