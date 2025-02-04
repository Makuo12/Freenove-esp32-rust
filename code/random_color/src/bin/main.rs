#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Level, Output}, ledc::{channel::{self, ChannelHW, ChannelIFace}, timer::{self, TimerIFace}, Ledc, LowSpeed}, rng::Trng, time::RateExtU32};
use esp_hal::main;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let ledc = Ledc::new(peripherals.LEDC);
    let pin2 = Output::new(peripherals.GPIO2, Level::Low);
    let pin0 = Output::new(peripherals.GPIO0, Level::Low);
    let pin4 = Output::new(peripherals.GPIO4, Level::Low);
    let mut rng = Trng::new(peripherals.RNG, peripherals.ADC1);
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
    .configure(timer::config::Config {
        duty: timer::config::Duty::Duty14Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: 50_u32.Hz(),
    })
    .unwrap();
    let mut pin0_channel0 = ledc.channel(channel::Number::Channel0, pin0);
    pin0_channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin2_channel1 = ledc.channel(channel::Number::Channel1, pin2);
    pin2_channel1
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin4_channel2 = ledc.channel(channel::Number::Channel2, pin4);
    pin4_channel2
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    let delay = Delay::new();
    loop {
        let random = &mut rng.random();
        let duty = *random % 100;
        pin0_channel0.set_duty_hw(duty as u32);
        pin2_channel1.set_duty_hw(duty as u32);
        pin4_channel2.set_duty_hw(duty as u32);
        delay.delay_micros(1000);
    }
    
}
