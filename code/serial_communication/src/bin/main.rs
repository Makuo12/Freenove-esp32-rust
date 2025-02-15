#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::uart::{Config, Uart};
use esp_hal::main;
use core::str::from_utf8;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut uart1 = Uart::new(
    peripherals.UART1,
    Config::default())
    .unwrap()
    .with_rx(peripherals.GPIO1)
    .with_tx(peripherals.GPIO2);
    uart1.write_bytes(b"Rust is calm for embedded development").unwrap();
    let mut bytes: [u8; 100] = [0;100];
    loop {
        uart1.read_bytes(&mut bytes).unwrap();
        esp_println::println!("esp32: {}", from_utf8(&bytes).unwrap())
    }
}
