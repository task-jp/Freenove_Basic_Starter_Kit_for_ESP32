#![no_std]
#![no_main]

use esp_backtrace as _;
// use esp_println::println;
use hal::{
    adc::{AdcConfig, Attenuation, ADC, ADC2},
    clock::ClockControl,
    dac,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    Delay,
};

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

    let pin25 = io.pins.gpio25.into_analog();
    let mut dac1 = dac::DAC1::dac(analog.dac1, pin25).unwrap();

    loop {
        let adc_value = nb::block!(adc2.read(&mut pin15)).unwrap();
        let dac_value = adc_value as u32 * 255 / 4095;
        dac1.write(dac_value as u8);
        esp_println::println!("adc_value: {}, dac_value: {}, voltage: {:2}V", adc_value, dac_value, adc_value as f32 / 4095.0 * 3.3);
        delay.delay_ms(1500u32);
    }
}
