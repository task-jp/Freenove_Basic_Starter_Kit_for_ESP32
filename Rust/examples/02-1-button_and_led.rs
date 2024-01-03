#![no_std]
#![no_main]

use esp_backtrace as _;
use hal::{peripherals::Peripherals, prelude::*, IO};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut pin_led = io.pins.gpio2.into_push_pull_output();
    let pin_button = io.pins.gpio13.into_pull_up_input();

    loop {
        if pin_button.is_low().unwrap() {
            pin_led.set_high().unwrap();
        } else {
            pin_led.set_low().unwrap();
        }
    }
}
