# hmc5883l-rs

[![Version](https://img.shields.io/crates/v/hmc5883l.svg)](https://crates.io/crates/hmc5883l)
[![Docs](https://docs.rs/hmc5883l/badge.svg)](https://docs.rs/hmc5883l)

Rust library for the HMC5883L magnetometer

This code lets you read x, y, z values from an HMC5883L magnetometer in Rust. This code will only work on Linux platforms and has only been tested on the Raspberry Pi 3 using this HMC5883L breakout: https://www.sparkfun.com/products/10530

The default I2C address for this board is 0x1E but if you have a different board or just the raw IC then check the datasheet for the appropriate address to use.

## Basic Usage

```rust
let mut mag = HMC5883L::new("/dev/i2c-1", 0x1E).unwrap();

loop {
    let (x, y, z) = mag.read().unwrap();
    println!("x={}, y={}, z={}", x, y, z);
    thread::sleep(Duration::from_millis(100));
}
```

## Wiring

You will need to connect SDA and SCL on the Pi to the corresponding pins on the magnetometer, as well as connecting 3.3V and GND.

## Running the examples

There are currently two examples. The `magnetometer` example shows the raw x, y, z values, and the `compass` example extends this by also calculating the approximate heading.

```rust
cargo run --example compass
```


You should see output like this. The values will change as your move the magnetometer.

```
x=7.0, y=19.9, z=-16.4 uT: heading=83.2
x=4.6, y=21.4, z=-15.8 uT: heading=90.4
x=4.0, y=21.8, z=-16.0 uT: heading=92.2
x=24.5, y=-21.3, z=-14.0 uT: heading=331.7
x=24.7, y=-9.5, z=-16.8 uT: heading=351.7
x=25.4, y=-8.5, z=-16.3 uT: heading=354.0
```