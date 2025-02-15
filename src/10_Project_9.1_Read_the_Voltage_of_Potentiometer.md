# Project 9.1 Read the Voltage of Potentiometer

In this project, we will use the ADC function of ESP32 to read the voltage value of the potentiometer and print it out through the serial monitor.
Remember that you can always find more information about this project in the Python or C instructional materials in the [material folder on github](https://github.com/Makuo12/Freenove-esp32-rust/tree/main/materials) to gain a better idea of how to arrange the various components.

## ADC

An ADC is an electronic integrated circuit used to convert analog signals such as voltages to digital or binary form consisting of 1s and 0s. The range of our ADC on ESP32-S3 is 12 bits, that means the resolution is 2^12=4096, and it represents a range (at 3.3V) will be divided equally to 4096 parts. The rage of analog values corresponds to ADC values. So the more bits the ADC has, the denser the partition of analog will be and the greater the precision of the resulting conversion.

## Component knowledge

Potentiometer
A potentiometer is a three-terminal resistor. Unlike the resistors that we have used thus far in our project which have a fixed resistance value, the resistance value of a potentiometer can be adjusted. A potentiometer is often made up by a resistive substance (a wire or carbon element) and movable contact brush. When the brush moves along the resistor element, there will be a change in the resistance of the potentiometer’s output side (3) (or change in the voltage of the circuit that is a part).

## Rotary potentiometer

Rotary potentiometer and linear potentiometer have similar function; their only difference is: the resistance is adjusted by rotating the potentiometer.

```rust
let mut adc_config = AdcConfig::new();
    let mut adc1_pin34= adc_config.enable_pin(peripherals.GPIO34, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc_config);
    loop {
        let result = adc1.read_oneshot(&mut adc1_pin34).unwrap();
        // To calculate the voltage
        let vol = (result as f32 * 3.3) / 4095.0;
        esp_println::println!("Voltage: {}, ADC: {}", vol, result) 
    }
```

### Initial Setup

```rust
let mut adc_config = AdcConfig::new();
```

Creates a new ADC configuration object that will be used to set up the ADC parameters.

### Pin Configuration

```rust
let mut adc1_pin34 = adc_config.enable_pin(peripherals.GPIO34, Attenuation::_11dB);
```

- Enables GPIO34 for ADC readings
- Sets attenuation to 11dB, which allows reading voltages up to about 3.3V
- The pin is connected to ADC1 (ESP32 has two ADC units)

### ADC Initialization

```rust
let mut adc1 = Adc::new(peripherals.ADC1, adc_config);
```

Creates a new ADC instance using ADC1 peripheral with the configured settings.

### Reading Loop

```rust
loop {
    let result = adc1.read_oneshot(&mut adc1_pin34).unwrap();
```

- Continuously reads ADC values
- `read_oneshot` takes one reading (as opposed to continuous sampling)
- Returns a value between 0-4095 (12-bit resolution)

### Voltage Calculation

```rust
let vol = (result as f32 * 3.3) / 4095.0;
```

- Converts raw ADC value to voltage
- 4095 is max value (2^12 - 1)
- 3.3V is the reference voltage
- Result is in volts

### Output

```rust
esp_println::println!("Voltage: {}, ADC: {}", vol, result)
```

Prints both the calculated voltage and raw ADC value.

### [Next: Soft light](11_Project_11.1_Soft_Light.md)
