# Project 2.1 Button & LED

In the project, we will control the LED state through a Push Button Switch. When the button is pressed, our
LED will turn ON, and when it is released, the LED will turn OFF. Remember that you can always find more information about this project in the Python or C instructional materials in the [material folder on github](https://github.com/Makuo12/Freenove-esp32-rust/tree/main/materials) to gain a better idea of how to arrange the various components.

## Requirements

You will need an esp32 and micro usb cable.
I will also be using visual studio code.

## Setup

Generate a template project

```shell
esp-generate --chip esp32 button_and_led
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
use esp_hal::gpio::{Input, Level, Output, Pull};
use esp_hal::main;
```

#### Entry point

We would need to define our own entry point because we don't have the main function

```rust
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
```

### Explanation

```rust
let peripherals = esp_hal::init(esp_hal::Config::default());
```

esp_hal::init() initializes the hardware abstraction layer (HAL) for the ESP microcontroller.
esp_hal::Config::default() provides the default configuration for initializing the HAL.

```rust
    let mut led = Output::new(peripherals.GPIO2, Level::Low);
```

we set the led as Output becuase we would be using the GPIO pin to send digital output signal to the LED

```rust
    let button = Input::new(peripherals.GPIO4, Pull::Up);
```

we set the GPIO4 as an Input because we want to know when a button is pressed. When the button is pressed down the signal in the state changes to low.

```rust
    loop {
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
```

Here when the button is pressed down we get an input signal which sets the GPIO to low and when it is low we set the led to high.

### [Next: Breathing LED](06_Project_4.1_Breathing_LED.md)
