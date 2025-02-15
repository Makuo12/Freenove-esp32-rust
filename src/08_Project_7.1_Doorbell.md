# Project 7.1 Doorbell

We will make this kind of doorbell: when the button is pressed, the buzzer sounds; and when the button is released, the buzzer stops sounding. Remember that you can always find more information about this project in the Python or C instructional materials in the [material folder on github](https://github.com/Makuo12/Freenove-esp32-rust/tree/main/materials) to gain a better idea of how to arrange the various components.

The code is the code/buzzer folder in the git repo of this book.

Buzzer
Buzzer is a sounding component, which is widely used in electronic devices such as calculator, electronic warning clock and alarm. Buzzer has two types: active and passive. Active buzzer has oscillator inside, which will sound as long as it is supplied with power. Passive buzzer requires external oscillator signal (generally use PWM with different frequency) to make a sound.

To make it easy we will be using an active buzzer. This make it possible for us to produce the sounds just like the same way we would on and off an LED.

## Crates used

```rust
use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Input, Level, Output}};
use esp_hal::main;
```

Remeber we always have to declare a main entry point of our application. This is important because we are not using std so rust won;t be able to do that automatically.

```rust
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
```

From the code above we have already see alot of this functionality so let talk about the logic.

```rust
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
```

When dealing with button pressing there is this kind of bounce effect, to handle this we usually use like a while loop so that for as long as the button is pressed the active buzzer will sound.

### [Next: Serial Read and Write](09_Project_8.2_Serial_Read_and_Write.md)
