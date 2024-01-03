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
    let pin_button = io.pins.gpio13.into_pull_up_input();

    loop {
        if pin_button.is_low().unwrap() {
            delay.delay_ms(20u32);
            if pin_button.is_low().unwrap() {
                pin_led.toggle().unwrap();
            }
            while pin_button.is_low().unwrap() {}
            delay.delay_ms(20u32);
            while pin_button.is_low().unwrap() {}
        }
    }
}
