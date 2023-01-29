# units-rs

This Rust library provides a comprehensive set of tools for working with standard international units (SI units). It includes a wide range of unit types and conversions, as well as support for composite units and unit prefixes. With this library, you can easily work with units such as meters, seconds, joules, and more, in a type-safe and intuitive way.

```rust
use units_si::{si};
use units::*;

fn main(){
    let length = si!(32m);
    let duration = si!(10ms);
    let velocity = length / duration;
    println!("Velocity: {}", velocity);
    
    let voltage = si!(10A * 100ohms);
    println!("Voltage: {}", voltage);
    
    //doing and ADC conversion
    let adc_value = 100_u16;
    let volts_per_bit = si!(4mV);
    let v_out = (adc_value as NativeType) * volts_per_bit;
    let r_top = si!(4kohms);
    let r_bottom = si!(1kohms);
    //calculate the input voltage before a voltage divider
    let sensor_voltage = v_out * ((r_top + r_bottom) /  r_bottom);
    println!("Sensor Voltage: {}", sensor_voltage);
}
```

Output:

```
Velocity: 3200 meters per second
Voltage: 1000 volts
Sensor Voltage: 2 volts
```

## Cargo

```
[dependencies]
units = { git = "https://github.com/tyler-gilbert/units-rs", tag = "v0.2.0" }
units-si = { git = "https://github.com/tyler-gilbert/units-rs", tag = "v0.2.0" }
```