#![no_std]
#![no_main]

use esp_backtrace as _;
// use esp_println::println;
use hal::{
    adc::{AdcConfig, Attenuation, ADC, ADC2},
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    Delay,
};
use num_traits::real::Real;

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

    loop {
        let adc_value = nb::block!(adc2.read(&mut pin15)).unwrap();
        let voltage = adc_value as f32 / 4095.0 * 3.3;
        let r_t = 10f32 * voltage / (3.3 - voltage);
        let temp_k = 1.0 / (1.0/(273.15 + 25.0) + (r_t / 10.0).log(2.7182818284590452) / 3950.0);
        let temp_c = temp_k - 273.15;
        esp_println::println!("ADC value: {}, Voltage: {:2}V, Temperature: {:2}C", adc_value, voltage, temp_c);
        delay.delay_ms(1000u32);
    }
}
