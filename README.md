# hmc5883l-rs
Rust library for the HMC5883L magnetometer

This code lets you read x, y, z values from an HMC5883L magnetometer in Rust. This code will only work on Linux platforms and has only been tested on the Raspberry Pi 3.

## Usage

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

## Running the example

To run the example:

```rust
cargo run --example example
```


You should see output like this. The values will change as your move the magnetometer.

```
x=498, y=583, z=262
x=478, y=568, z=303
x=451, y=579, z=325
x=454, y=574, z=328
```