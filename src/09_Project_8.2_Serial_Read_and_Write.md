# Project 8.2 Serial Read and Write

Here we will be communicating between our esp32 microcontroller and our computer. To perform this operation we will be using UART.

## UART

UART (Universal Asynchronous Receiver-Transmitter) is a hardware communication protocol used for serial communication between devices. Let's break down its key aspects:
Key Components:

TX (Transmit) line: Sends data
RX (Receive) line: Receives data
Ground line: Common reference voltage

Key Characteristics:

Asynchronous: No separate clock signal is needed - both devices must agree on timing beforehand
Full-duplex: Can send and receive data simultaneously
Uses start/stop bits to frame each byte of data
Common baud rates: 9600, 115200 bps (bits per second)

Remember that you can always find more information about this project in the Python or C instructional materials in the [material folder on github](https://github.com/Makuo12/Freenove-esp32-rust/tree/main/materials) to gain a better idea of how to arrange the various components.

The code is the code/serial_commuication folder in the git repo of this book.

Using the esp_hal crate it is actually quite simple.

```rust
use esp_backtrace as _;
use esp_hal::uart::{Config, Uart};
use esp_hal::main;
use core::str::from_utf8;
```

We would be using the Config from the uart crate just to use the default configurations and then we would use the Uart struct which would return us a blocking Uart that we can use to perform the operations of reading from the computer to our microcontroller and writing from our microcontroller to the computer.

```rust
    let mut uart1 = Uart::new(
    peripherals.UART1,
    Config::default())
    .unwrap()
    .with_rx(peripherals.GPIO1)
    .with_tx(peripherals.GPIO2);
```

We define the pins we would use for rx to receive data from the device and tx to transmit data to a device. Most of the time there are special pins configure to be used for tx and rx so just check out your board's layout if you are having any issues or post an issue.

```rust
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

```rust
    uart1.write_bytes(b"Rust is calm for embedded development").unwrap();
    let mut bytes: [u8; 100] = [0;100];
    loop {
        uart1.read_bytes(&mut bytes).unwrap();
        esp_println::println!("esp32: {}", from_utf8(&bytes).unwrap())
    }
```

Next we perform the operations. To write to a device we use the `write_bytes` method. To read we use the `read_bytes` method. We pass in a bytes slice that we want it to store the data that we read from the computer and then we print it out.

### [Next: Read voltage ADC](10_Project_9.1_Read_the_Voltage_of_Potentiometer.md)
