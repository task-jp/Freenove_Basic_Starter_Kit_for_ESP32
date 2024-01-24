#![no_std]
#![no_main]

use esp_backtrace as _;
// use esp_println::println;
use core::fmt::Write;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, Uart};
use heapless::String;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let mut delay = Delay::new(&clocks);

    let mut uart0 = Uart::new(peripherals.UART0, &clocks);

    writeln!(uart0, "ESP32 initialization completed!").unwrap();
    writeln!(uart0, "Please input some characters.").unwrap();
    writeln!(uart0, "select \"Newline\" below and click send button.").unwrap();

    let mut input_string: String<80> = String::new();
    loop {
        while let nb::Result::Ok(c) = uart0.read() {
            match c {
                0x0D => {
                    writeln!(uart0, "inputString: {}", input_string).unwrap();
                    input_string.clear();
                }
                _ => {
                    input_string.push(c as char);
                }
            }
        }
    }
}
