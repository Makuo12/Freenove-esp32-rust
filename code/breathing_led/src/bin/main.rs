#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay, gpio::{Input, Level, Output, OutputOpenDrain, Pull}, prelude::*, rtc_cntl::Rtc,
    ledc::{channel::{self, Channel}, timer::{self, Timer as LCD_Timer}, LSGlobalClkSource, Ledc, LowSpeed};
};
use esp_println::println;



#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    // columns
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
    .configure(timer::config::Config {
        duty: timer::config::Duty::Duty14Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: 50_u32.Hz(),
    })
    .unwrap();
    let mut pin2_channel1 = board.ledc.channel(channel::Number::Channel1, peripherals.GPIO2);
    pin2_channel1
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    loop {
        for i in 0..255 {
            pin2_channel1.set_duty(i);
        }
    }

}
