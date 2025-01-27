# Project 1.2 Blink

This project would consist of a Esp32, a breadboard, one LED, a resistor, and two jumper wires. Remember that you can always find more information about this project in the Python or C instructional materials in the material folder on github to gain a better idea of how to arrange the various components.

## Requirements

You will need an esp32 and micro usb cable.
I will also be using visual studio code.

## Setup

Generate a template project

```shell
esp-generate --chip esp32 blinky_two
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
use esp_backtrace as_;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Level, Output, Pin, Pull},
    prelude::*,
};
```

#### Entry point

We would need to define our own entry point because we don't have the main function

```rust
# [entry]
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
```

### Explanation

```rust
let peripherals = esp_hal::init(esp_hal::Config::default());
```

esp_hal::init() initializes the hardware abstraction layer (HAL) for the ESP microcontroller.
esp_hal::Config::default() provides the default configuration for initializing the HAL.

```rust
let mut led2 = Output::new(peripherals.GPIO2.degrade(), Level::Low)
```

we set the led as Output becuase we would be using the GPIO pin to send digital output signal to the LED
I would be still be using GPIO2 but this time I will be providing voltage to the resistor that is providing resistance to the led.

`Output::new()` creates an output pin for GPIO2, which will control an LED.
`peripherals.GPIO2`: Refers to GPIO pin 2 of the ESP microcontroller.
`.degrade()`: Converts the specific GPIO pin object into a more generic type like AnyPin.
`Level::Low`: Initializes the GPIO pin to a low state (i.e., LED off).

```rust
loop {
    led2.set_high();
    delay.delay_millis(100);
    led2.set_low();
    delay.delay_millis(100);
}
```

This makes it possible to create a bink effect as we would toggle various state (off and on) the delay ensures that we get to see the led as it is blinking.

What happens
LED turns on for 100 milliseconds.
LED turns off for 100 milliseconds.
This creates a blinking effect with a frequency of 5 Hz (a blink every 200 ms, which is 5 blinks per second).

