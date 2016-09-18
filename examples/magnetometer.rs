extern crate hmc5883l;
use hmc5883l::*;

use std::thread;
use std::time::Duration;

fn main() {

    let mut mag = HMC5883L::new("/dev/i2c-1", 0x1E).unwrap();

    let gauss_lsb_xy = 1100.0;
    let gauss_lsb_z  =  980.0;
    let declination_angle = 0.22; // you need to set this based on your location

    loop {
        let (x, y, z) = mag.read().unwrap();

        // convert to micro-teslas
        let (x, y, z) = (x/gauss_lsb_xy*100.0, y/gauss_lsb_xy*100.0, z/gauss_lsb_z*100.0);

        println!("x={}, y={}, z={}", x, y, z);
        thread::sleep(Duration::from_millis(500));
    }

}
