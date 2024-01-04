#![no_std]
#![no_main]

use esp_backtrace as _;

use hal::{
    clock::ClockControl,
    ledc::{self, channel, timer, LSGlobalClkSource, LowSpeed, LEDC},
    peripheral::Peripheral,
    peripherals::Peripherals,
    prelude::*,
    Delay, IO,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led_pin0 = io.pins.gpio15.into_push_pull_output();
    let mut led_pin1 = io.pins.gpio2.into_push_pull_output();
    let mut led_pin2 = io.pins.gpio0.into_push_pull_output();
    let mut led_pin3 = io.pins.gpio4.into_push_pull_output();
    let mut led_pin4 = io.pins.gpio5.into_push_pull_output();
    let mut led_pin5 = io.pins.gpio18.into_push_pull_output();
    let mut led_pin6 = io.pins.gpio19.into_push_pull_output();
    let mut led_pin7 = io.pins.gpio21.into_push_pull_output();
    let mut led_pin8 = io.pins.gpio22.into_push_pull_output();
    let mut led_pin9 = io.pins.gpio23.into_push_pull_output();

    todo!("Implement the rest of the example")
}
