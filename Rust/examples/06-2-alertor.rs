#![no_std]
#![no_main]

use esp_backtrace as _;
// use esp_println::println;

use hal::{
    clock::ClockControl,
    ledc::{channel, timer, LSGlobalClkSource, LowSpeed, LEDC},
    peripherals::Peripherals,
    prelude::*,
    Delay, IO,
};
use num_traits::real::Real;
use fugit::HertzU32;

fn set_freq(ledc: &LEDC<'_>, pin: &mut hal::gpio::GpioPin<hal::gpio::Output<hal::gpio::PushPull>, 13>, freq: HertzU32, duty_pct: u8) {
    let mut lstimer0 = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty13Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: freq,
        })
        .unwrap();
    let mut channel0 = ledc.get_channel(channel::Number::Channel0,  pin);
    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: duty_pct,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut pin_buzzer = io.pins.gpio13.into_push_pull_output();
    let pin_button = io.pins.gpio4.into_pull_up_input();

    let mut ledc = LEDC::new(peripherals.LEDC, &clocks);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    set_freq(&ledc, &mut pin_buzzer, 2000u32.Hz(), 50);
    delay.delay_ms(300u32);

    loop {
        if pin_button.is_low().unwrap() {
            for x in 0..36 {
                let freq = 2000f32 + 500f32 * (x as f32 * 3.14 / 180f32).sin();
                set_freq(&ledc, &mut pin_buzzer, fugit::HertzU32::from_raw(freq as u32), 50);
                delay.delay_ms(1u32);
            }
        } else {
            set_freq(&ledc, &mut pin_buzzer, 1u32.kHz(), 0);
        }
    }
}
