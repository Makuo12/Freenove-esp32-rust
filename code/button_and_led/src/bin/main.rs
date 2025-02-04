#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::gpio::{Input, Level, Output, Pull};
use esp_hal::main;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Set LED GPIOs as an output:
    let mut led = Output::new(peripherals.GPIO2, Level::Low);
    let button = Input::new(peripherals.GPIO4, Pull::Up);
    loop {
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
