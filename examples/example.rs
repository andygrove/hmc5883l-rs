extern crate hmc5883l;

use hmc5883l::*;

fn main() {

    let mag = HMC5883L::new("/dev/i2c-1", 0x1E);

    loop {
        let (x, y, z) = mag.read();
        println!("x={}, y={}, z={}", x, y, z);
    }

}