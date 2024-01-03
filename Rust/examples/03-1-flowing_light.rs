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
    let mut led0 = io.pins.gpio15.into_push_pull_output();
    let mut led1 = io.pins.gpio2.into_push_pull_output();
    let mut led2 = io.pins.gpio0.into_push_pull_output();
    let mut led3 = io.pins.gpio4.into_push_pull_output();
    let mut led4 = io.pins.gpio5.into_push_pull_output();
    let mut led5 = io.pins.gpio18.into_push_pull_output();
    let mut led6 = io.pins.gpio19.into_push_pull_output();
    let mut led7 = io.pins.gpio21.into_push_pull_output();
    let mut led8 = io.pins.gpio22.into_push_pull_output();
    let mut led9 = io.pins.gpio23.into_push_pull_output();

    macro_rules! flash_leds {
        ($($gpio:ident),+) => {
            $(
                $gpio.toggle().unwrap();
                delay.delay_ms(100u32);
                $gpio.toggle().unwrap();
            )+
        };
    }
    loop {
        flash_leds!(
            led0,
            led1,
            led2,
            led3,
            led4,
            led5,
            led6,
            led7,
            led8,
            led9
        );
        flash_leds!(
            led9,
            led8,
            led7,
            led6,
            led5,
            led4,
            led3,
            led2,
            led1,
            led0
        );
    }
}
