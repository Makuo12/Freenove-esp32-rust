#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::ledc::timer::TimerIFace as _;
use esp_hal::ledc::{channel, timer, Ledc, LowSpeed};
use esp_hal::main;
use esp_hal::time::RateExtU32 as _;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut adc_config = AdcConfig::new();
    let mut adc1_pin34= adc_config.enable_pin(peripherals.GPIO34, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc_config);
    let ledc = Ledc::new(peripherals.LEDC);
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
    .configure(timer::config::Config {
        duty: timer::config::Duty::Duty14Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: 50_u32.Hz(),
    })
    .unwrap();
    let mut pin2_channel1 = ledc.channel(channel::Number::Channel1, peripherals.GPIO2);
    pin2_channel1
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    loop {
        let result = adc1.read_oneshot(&mut adc1_pin34).unwrap();
        let value = remap(result, 0, 4095, 0, 1023);
        pin2_channel1.set_duty(value as u8).unwrap();
    }
}

fn remap(adc_value: u16, old_min: u16, old_max: u16, new_min: u16, new_max: u16) -> u16 {
    return adc_value * (new_max-new_min) / (old_max-old_min)
}