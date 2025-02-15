# Project 4.1 Breathing LED

Breathing light, that is, LED is turned from off to on gradually, and gradually from on to off, just like "breathing".
So, how to control the brightness of a LED? We will use PWM to achieve this target. Remember that you can always find more information about this project in the Python or C instructional materials in the [material folder on github](https://github.com/Makuo12/Freenove-esp32-rust/tree/main/materials) to gain a better idea of how to arrange the various components.

## PWM (Pulse Width Modulation)

This is a rectangular wave form. For each cycle we have an on state and an off state. By combining this two states we can produce an analog signal (artificial analog signal). When the circuit is in an on state we can call it Pulse width and when it is in an off state we can call it space width.
t1 -> would represent the time period for Pulse width (PW)
t2 -> would represent the time period for space width (SW)

```math
Cycle time = PW + SW
```

So we can find the frequency = 1 / (cycle time)

## Duty Cycle

This is another important term in PWM. It is a percentage that represents the time period when the state is on.

```math
duty cycle = PW / (cycle time) * 100
```

When the duty cycle is at 50% then the waveform becomes a square wave.

```rust
    let mut pin2_channel1 = ledc.channel(channel::Number::Channel1, peripherals.GPIO2);
    pin2_channel1
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

```

We see that here we set the duty_pct (duty percentage) to 10% that means it will be on an on state 10% of the time.

The PWM pin output mode of ESP32 is not the same as the traditional controller. It controls each
parameter of PWM by controlling the PWM channel. Any number of GPIO can be connected with the PWM
channel to output PWM.

```rust
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{ledc::{channel::{self, ChannelIFace}, timer::{self, TimerIFace}, Ledc, LowSpeed}, time::RateExtU32};
use esp_hal::main;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let ledc = Ledc::new(peripherals.LEDC);
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
    .configure(timer::config::Config {
        duty: timer::config::Duty::Duty14Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: 50_u32.Hz(),
    })
    .unwrap();
    let mut pin2_channel1 = ledc.channel(channel::Number::Channel1, peripherals.GPIO2);
    pin2_channel1
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    loop {
        for i in 0..255 {
            let _ = pin2_channel1.set_duty(i);
        }
    }

}
```

```rust
    let ledc = Ledc::new(peripherals.LEDC);
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
    .configure(timer::config::Config {
        duty: timer::config::Duty::Duty14Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: 50_u32.Hz(),
    })
    .unwrap();
```

Counting helps us keep track of our resolution. Resolutions helps to know that are the possible values for our duty cycle percentage.

Imagine we are counting from 1 to 8. Let say that at the start the duty cycle is 100% this means from 1 to 8 the states is always on. Now lets say we want 50% duty cycle we see that maybe from 1 to 4 the circuit is on while 5 to 8 it remains off. However, you will also notice that from 1 to 8 there is only a few number of duty cycles percentages we can use (100%, 87.5%, 75%, 62.5%, 50%, 37.5%, 25%, 12.5%, and 0%). So if we want a duty cycle of 92% it would not be possible using this resolution.

To fix this we would want to create our count to be longer maybe we can make the resolution 32. Note that the drawback of increasing resolution is that it takes longer to complete a cycle.

The Frequency is how many times the cycle repeats per second.

### [Next: Random Color Light](07_Project_5.1_Random_Color_Light.md)
