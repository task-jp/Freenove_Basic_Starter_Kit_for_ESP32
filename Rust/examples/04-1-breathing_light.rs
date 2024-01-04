#![no_std]
#![no_main]

use esp_backtrace as _;

use hal::{
    clock::ClockControl,
    ledc::{channel, timer, LSGlobalClkSource, LowSpeed, LEDC},
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
    let mut pin_led = io.pins.gpio2.into_push_pull_output();

    let mut ledc = LEDC::new(peripherals.LEDC, &clocks);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty13Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: hal::prelude::_fugit_RateExtU32::Hz(1000),
        })
        .unwrap();
    let mut channel0 = ledc.get_channel(channel::Number::Channel0, &mut pin_led);
    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 8,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    loop {
        for i in 0..=100 {
            channel0.set_duty(i).unwrap();
            delay.delay_ms(20u32);
        }
        for i in 0..=100 {
            channel0.set_duty(100 - i).unwrap();
            delay.delay_ms(20u32);
        }
    }
}
