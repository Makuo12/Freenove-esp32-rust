#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Level, Output}, prelude::*};
use log::info;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Set LED GPIOs as an output:
    let mut led2 = Output::new(peripherals.GPIO2.degrade(), Level::Low);

    let delay = Delay::new();
    loop {
        led2.toggle();
        delay.delay_millis(50);
    }
}
