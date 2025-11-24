# sci-units

This Rust library provides a comprehensive set of tools for working with standard international units (SI units). It includes a wide range of unit types and conversions, as well as support for composite units and unit prefixes. With this library, you can easily work with units such as meters, seconds, joules, and more, in a type-safe and intuitive way.

```rust
use sci_units::*;

fn main(){
    let length = Length::new(32.0);
    let duration = Time::new(0.01);
    let velocity = length / duration;
    println!("Velocity: {}", velocity);

    let voltage = ElectricCurrent::new(10.0) * ElectricResistance::new(100.0);
    println!("Voltage: {}", voltage);

    //doing and ADC conversion
    let adc_value = Scalar::new(100.0);
    let volts_per_bit = ElectricPotential::new(0.004);
    let v_out = adc_value * volts_per_bit;
    let r_top = ElectricResistance::new(4000.0);
    let r_bottom = ElectricResistance::new(1000.0);
    //calculate the input voltage before a voltage divider
    let sensor_voltage = v_out * ((r_top + r_bottom) /  r_bottom);
    println!("Sensor Voltage: {}", sensor_voltage);
}
```

Output:

```
Velocity: 3200 meters/second
Voltage: 1000 volts
Sensor Voltage: 2 volts
```
