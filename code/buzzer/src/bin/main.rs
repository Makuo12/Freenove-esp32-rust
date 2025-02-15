#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Input, Level, Output}};
use esp_hal::main;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let button12 = Input::new(peripherals.GPIO12, esp_hal::gpio::Pull::Up);
    let mut pin4 = Output::new(peripherals.GPIO4, Level::Low);
    let delay = Delay::new();
    loop {
        if button12.is_low() {
            delay.delay_millis(50);
            if button12.is_low() {
                // We play the buzzar
                pin4.set_high();
            }
            while button12.is_low() {
                pin4.set_high();
            }
        } else {
            pin4.set_low();
        }
    }
}
