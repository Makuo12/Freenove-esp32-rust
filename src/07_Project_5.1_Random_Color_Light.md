# Project 5.1 Random Color Light

In this project, we will make a multicolored LED. And we can control RGB LED to switch different colors
automatically. Remember that you can always find more information about this project in the Python or C instructional materials in the [material folder on github](https://github.com/Makuo12/Freenove-esp32-rust/tree/main/materials) to gain a better idea of how to arrange the various components.

RGB LED has integrated 3 LEDs that can respectively emit red, green and blue light. And it has 4 pins. The
long pin (1) is the common port, that is, 3 LED 's positive or negative port. The RGB LED with common positive
port and its symbol is shown below. We can make RGB LED emit various colors of light by controlling these 3
LEDs to emit light with different brightness.

Red, green, and blue are known as three primary colors. When you combine these three primary-color lights
with different brightness, it can produce almost all kinds of visible lights. Computer screens, single pixel of cell
phone screen, neon, and etc. are working under this principle.

If we use three 8-bit PWMs to control the RGB LED, in theory, we can create 28*28*28=16777216 (16 million)
colors through different combinations.

We will be using some of the same objects that we used in [Project 6](06_Project_4.1_Breathing_LED.md)

```rust
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Level, Output}, ledc::{channel::{self, ChannelHW, ChannelIFace}, timer::{self, TimerIFace}, Ledc, LowSpeed}, rng::Trng, time::RateExtU32};
use esp_hal::main;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let ledc = Ledc::new(peripherals.LEDC);
    let pin2 = Output::new(peripherals.GPIO2, Level::Low);
    let pin0 = Output::new(peripherals.GPIO0, Level::Low);
    let pin4 = Output::new(peripherals.GPIO4, Level::Low);
    let mut rng = Trng::new(peripherals.RNG, peripherals.ADC1);
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
    .configure(timer::config::Config {
        duty: timer::config::Duty::Duty14Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: 50_u32.Hz(),
    })
    .unwrap();
    let mut pin0_channel0 = ledc.channel(channel::Number::Channel0, pin0);
    pin0_channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin2_channel1 = ledc.channel(channel::Number::Channel1, pin2);
    pin2_channel1
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin4_channel2 = ledc.channel(channel::Number::Channel2, pin4);
    pin4_channel2
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    let delay = Delay::new();
    loop {
        let random = &mut rng.random();
        let duty = *random % 100;
        pin0_channel0.set_duty_hw(duty as u32);
        pin2_channel1.set_duty_hw(duty as u32);
        pin4_channel2.set_duty_hw(duty as u32);
        delay.delay_micros(1000);
    }
    
}
```

`let mut rng = Trng::new(peripherals.RNG, peripherals.ADC1);` Here we use the Trng to be able to generate random number for the random color channel.

```rust
    let mut pin0_channel0 = ledc.channel(channel::Number::Channel0, pin0);
    pin0_channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
```

Here we configure to channel to use using the LEDC provided from the esp_hal crate.

```rust
        let random = &mut rng.random();
        let duty = *random % 100;
        pin0_channel0.set_duty_hw(duty as u32);
        pin2_channel1.set_duty_hw(duty as u32);
        pin4_channel2.set_duty_hw(duty as u32);
        delay.delay_micros(1000);
```

Here we set the duty. hw means hardware using this we can set values higher than u8 up to u32.