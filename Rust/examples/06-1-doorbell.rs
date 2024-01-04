#![no_std]
#![no_main]

use esp_backtrace as _;
// use esp_println::println;

use hal::{peripherals::Peripherals, prelude::*, IO};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut pin_buzzer = io.pins.gpio13.into_push_pull_output();
    let pin_button = io.pins.gpio4.into_pull_up_input();

    loop {
        if pin_button.is_low().unwrap() {
            pin_buzzer.set_high().unwrap();
        } else {
            pin_buzzer.set_low().unwrap();
        }
    }
}
