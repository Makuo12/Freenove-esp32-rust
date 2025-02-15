#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::main;

#[main]
fn main() -> ! {
    // generator version: 0.2.2

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    let mut adc_config = AdcConfig::new();
    let mut adc1_pin34= adc_config.enable_pin(peripherals.GPIO34, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc_config);
    loop {
        let result = adc1.read_oneshot(&mut adc1_pin34).unwrap();
        // To calculate the voltage
        let vol = (result as f32 * 3.3) / 4095.0;
        esp_println::println!("Voltage: {}, ADC: {}", vol, result) 
    }

}
