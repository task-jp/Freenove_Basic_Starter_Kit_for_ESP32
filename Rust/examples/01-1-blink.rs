#![no_std]
#![no_main]

use esp_backtrace as _;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut pin_led = io.pins.gpio2.into_push_pull_output();

    loop {
        pin_led.set_high().unwrap();
        delay.delay_ms(1000u32);
        pin_led.set_low().unwrap();
        delay.delay_ms(1000u32);
    }
}
