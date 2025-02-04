# Project 1.1 Blink (Important)

We will start with simple “Blink” project.

## Requirements

You will need an esp32 and micro usb cable.
I will also be using visual studio code.

## Setup

Generate a template project

```shell
esp-generate --chip esp32 blinky
```

Select the config option you feel you might need, but for now all we need is the Optional editor config files for rust-analyzer.
This would help add intellisense to the project and help us with bugs in our code.

## Code

To start off we would need to add the no_std and no_main commands to the top of my `main.rs` file

```rust
# ![no_std]
# ![no_main]
```

The #![no_std] attribute in Rust indicates that your code does not depend on the standard library (std). Instead, it relies only on the core library, which is a subset of the standard library designed for low-level and embedded systems programming.

The #![no_main] attribute in Rust is used to tell the compiler that your program does not have the usual main function entry point. Instead, you define a custom entry point, which is typically necessary in low-level programming scenarios like embedded systems, operating systems, or bare-metal applications.

### The libaries we would use

```rust
use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::gpio::{Level, Output};
```

#### Entry point

We would need to define our own entry point because we don't have the main function

```rust
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
```

### Explanation

```rust
    let peripherals = esp_hal::init(esp_hal::Config::default());
```

esp_hal::init() initializes the hardware abstraction layer (HAL) for the ESP microcontroller.
esp_hal::Config::default() provides the default configuration for initializing the HAL.

```rust
    let mut led2 = Output::new(peripherals.GPIO2, Level::Low);
```

we set the led as Output becuase we would be using the GPIO pin to send digital output signal to the LED
I used GPIO2 because that is what is connected to the LED on the esp32 development board.

Note: it is also possible to see peripherals.GPIO2.degrade() in some applications. This was used to just put the pin as anypin so it can be used easily but with the latest esp release there is no need. It is not even the library anymore.

`Output::new()` creates an output pin for GPIO2, which will control an LED.
`peripherals.GPIO2`: Refers to GPIO pin 2 of the ESP microcontroller.
`.degrade()`: Converts the specific GPIO pin object into a more generic type like AnyPin.
`Level::Low`: Initializes the GPIO pin to a low state (i.e., LED off).

```rust
loop {
    led2.toggle();
    delay.delay_millis(50);
}
```

This makes it possible to create a bink effect as we would toggle various state (off and on) the delay ensures that we get to see the led as it is blinking.

### [Next: Blink 2](04_Project_1.2_Blink.md)
