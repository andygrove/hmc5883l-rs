extern crate hmc5883l;
use hmc5883l::*;

use std::thread;
use std::time::Duration;

fn main() {

    let mut mag = HMC5883L::new("/dev/i2c-1", 0x1E).unwrap();

    loop {
        let (x, y, z) = mag.read().unwrap();
        println!("x={}, y={}, z={}", x, y, z);
        thread::sleep(Duration::from_millis(100));
    }

}