# Project 10.1 Read Touch Sensor

In this project, we will make a soft light. We will use an ADC Module to read ADC values of a potentiometer and map it to duty cycle of the PWM used to control the brightness of a LED. Then you can change the brightness of a LED by adjusting the potentiometer.

Remember that you can always find more information about this project in the Python or C instructional materials in the [material folder on github](https://github.com/Makuo12/Freenove-esp32-rust/tree/main/materials) to gain a better idea of how to arrange the various components.

The code is the code/soft_light folder in the git repo of this book.

## Code review

First, we're working with two main things here: an ADC (which reads voltage) and LED control (which controls brightness). Think of the ADC like a voltage meter and the LED control like a dimmer switch.

The first part is similar to what we saw before - setting up the ADC:

```rust
let peripherals = esp_hal::init(esp_hal::Config::default());
let mut adc_config = AdcConfig::new();
let mut adc1_pin34 = adc_config.enable_pin(peripherals.GPIO34, Attenuation::_11dB);
let mut adc1 = Adc::new(peripherals.ADC1, adc_config);
```

Now here's where it gets interesting - we're setting up LED control using something called LEDC (LED Control):

```rust
let ledc = Ledc::new(peripherals.LEDC);
let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
```

Think of this like setting up a special dimmer switch. The timer is like the mechanism inside the dimmer that controls how fast it can change brightness.

We configure this timer with some specific settings:

```rust
lstimer0.configure(timer::config::Config {
    duty: timer::config::Duty::Duty14Bit,
    clock_source: timer::LSClockSource::APBClk,
    frequency: 50_u32.Hz(),
})
```

This is like telling our dimmer "Hey, I want you to be able to have 14 bits worth of brightness levels (that's 16,384 levels!), and I want you to update 50 times per second."

Then we set up the actual LED pin:

```rust
let mut pin2_channel1 = ledc.channel(channel::Number::Channel1, peripherals.GPIO2);
pin2_channel1.configure(channel::config::Config {
    timer: &lstimer0,
    duty_pct: 10,
    pin_config: channel::config::PinConfig::PushPull,
})
```

This connects our dimmer mechanism to an actual LED on pin GPIO2.

In the main loop, here's where the magic happens:

```rust
loop {
    let result = adc1.read_oneshot(&mut adc1_pin34).unwrap();
    let value = remap(result, 0, 4095, 0, 1023);
    pin2_channel1.set_duty(value as u8).unwrap();
}
```

We:

1. Read the voltage (result will be between 0 and 4095)
2. Convert that number to a brightness level (between 0 and 1023)
3. Set the LED brightness to that level

The `remap` function is like a number converter:

```rust
fn remap(adc_value: u16, old_min: u16, old_max: u16, new_min: u16, new_max: u16) -> u16 {
    return adc_value * (new_max-new_min) / (old_max-old_min)
}
```

If you imagine a number line, this takes a position on one line (0-4095) and finds the equivalent position on another line (0-1023). It's like saying "if this number is 25% of the way from 0 to 4095, what number is 25% of the way from 0 to 1023?"

The end result? As you change the voltage on pin 34 (maybe with a potentiometer or sensor), the LED brightness on pin 2 changes proportionally. It's like having a voltage-controlled dimmer switch!
