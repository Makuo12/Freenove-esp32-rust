#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::gpio::{Level, Output};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Set LED GPIOs as an output:
    let mut led2 = Output::new(peripherals.GPIO2, Level::Low);

    let delay = Delay::new();
    loop {
        led2.toggle();
        delay.delay_millis(50);
    }
}