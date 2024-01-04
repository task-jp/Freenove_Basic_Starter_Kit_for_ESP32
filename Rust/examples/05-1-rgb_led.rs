#![no_std]
#![no_main]

use esp_backtrace as _;

use hal::{
    clock::ClockControl,
    ledc::{channel, timer, LSGlobalClkSource, LowSpeed, LEDC},
    peripherals::Peripherals,
    prelude::*,
    Delay, Rng, IO,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut pin_led_r = io.pins.gpio15.into_push_pull_output();
    let mut pin_led_g = io.pins.gpio2.into_push_pull_output();
    let mut pin_led_b = io.pins.gpio4.into_push_pull_output();

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
    let mut red = ledc.get_channel(channel::Number::Channel0, &mut pin_led_r);
    red.configure(channel::config::Config {
        timer: &lstimer0,
        duty_pct: 8,
        pin_config: channel::config::PinConfig::PushPull,
    })
    .unwrap();
    let mut green = ledc.get_channel(channel::Number::Channel1, &mut pin_led_g);
    green
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 8,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    let mut blue = ledc.get_channel(channel::Number::Channel2, &mut pin_led_b);
    blue.configure(channel::config::Config {
        timer: &lstimer0,
        duty_pct: 8,
        pin_config: channel::config::PinConfig::PushPull,
    })
    .unwrap();

    let mut rng = Rng::new(peripherals.RNG);
    loop {
        red.set_duty(rng.random() as u8 % 100).unwrap();
        green.set_duty(rng.random() as u8 % 100).unwrap();
        blue.set_duty(rng.random() as u8 % 100).unwrap();
        delay.delay_ms(250u32);
    }
}
