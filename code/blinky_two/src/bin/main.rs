#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::prelude::*;
use log::info;

#[entry]
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Input, Level, Output, Pull}, prelude::*};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Set LED GPIOs as an output:
    let mut led2 = Output::new(peripherals.GPIO2.degrade(), Level::Low);

    let delay = Delay::new();
    loop {
        led2.set_high();
        delay.delay_millis(100);
        led2.set_low();
        delay.delay_millis(100);
    }
}
