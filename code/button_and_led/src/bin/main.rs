#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay, gpio::{Input, Level, Output, Pull};
use esp_hal::main;
use log::info;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Set LED GPIOs as an output:
    let mut led = Output::new(peripherals.GPIO2.degrade(), Level::Low);
    let mut button = Input::new(peripherals.GPIO4.degrade(), Pull::Up);
    let delay = Delay::new();
    loop {
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
