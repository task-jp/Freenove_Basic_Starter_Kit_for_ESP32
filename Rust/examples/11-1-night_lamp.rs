#![no_std]
#![no_main]

use esp_backtrace as _;
// use esp_println::println;
use hal::{
    adc::{AdcConfig, Attenuation, ADC, ADC2},
    clock::ClockControl,
    gpio::IO,
    ledc::{channel, timer, HighSpeed, LSGlobalClkSource, LEDC},
    peripherals::Peripherals,
    prelude::*,
    Delay,
};

const LIGHT_MIN: u32 = 372;
const LIGHT_MAX: u32 = 2048;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let analog = peripherals.SENS.split();

    let mut adc2_config = AdcConfig::new();
    // pin 4 is not usable for adc in esp32-hal-common-0.13.1
    // https://github.com/esp-rs/esp-hal/commit/944ce9a33e5629fd7b08050290253f8ce144022d
    let mut pin15 =
        adc2_config.enable_pin(io.pins.gpio15.into_analog(), Attenuation::Attenuation11dB);
    let mut adc2 = ADC::<ADC2>::adc(analog.adc2, adc2_config).unwrap();

    let pin25 = io.pins.gpio25.into_push_pull_output();
    let mut ledc = LEDC::new(peripherals.LEDC, &clocks);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut hstimer0 = ledc.get_timer::<HighSpeed>(timer::Number::Timer0);
    hstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty13Bit,
            clock_source: timer::HSClockSource::APBClk,
            frequency: 8u32.kHz(),
        })
        .unwrap();
    let mut channel0 = ledc.get_channel(channel::Number::Channel0, pin25);
    channel0
        .configure(channel::config::Config {
            timer: &hstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    let mut last_value: u8 = 0;
    loop {
        let adc_value = nb::block!(adc2.read(&mut pin15)).unwrap() as u32;
        let adc_value = adc_value.min(LIGHT_MAX).max(LIGHT_MIN);
        let adc_value = (adc_value - LIGHT_MIN) * 100 / (LIGHT_MAX - LIGHT_MIN);
        let adc_value = adc_value as u8;

        if last_value != adc_value {
            esp_println::println!("{} -> {}", last_value, adc_value,);
            if let Ok(_) = channel0.set_duty(adc_value) {
                last_value = adc_value;
            }
        }
        delay.delay_ms(100u32);
    }
}
