#![no_std]
#![no_main]

use esp_backtrace as _;
// use esp_println::println;
use core::fmt::Write;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, Uart};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let mut delay = Delay::new(&clocks);

    let mut uart0 = Uart::new(peripherals.UART0, &clocks);
    writeln!(uart0, "ESP32 initialization completed!").unwrap();

    loop {
        writeln!(uart0, "Hello, world!").unwrap();
        delay.delay_ms(1000u32);
    }
}
