extern crate hmc5883l;
use hmc5883l::*;

use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

fn main() {

    let mut mag = HMC5883L::new("/dev/i2c-1", 0x1E).unwrap();

    let gauss_lsb_xy = 1100.0;
    let gauss_lsb_z  =  980.0;

    loop {
        let (x, y, z) = mag.read().unwrap();

        // convert to micro-teslas
        let (x, y, z) = (x/gauss_lsb_xy*100.0, y/gauss_lsb_xy*100.0, z/gauss_lsb_z*100.0);

        let mut heading = x.atan2(y);

        if heading < 0.0 {
            heading += 2.0 * PI;
        }

        if heading > 2.0 * PI {
            heading -= 2.0 * PI;
        }

        // Convert radians to degrees for readability.
        heading = heading * 180.0 / PI;

//        println!("x={}, y={}, z={} uT: heading={:.*}", x, y, z, 1, heading);
        println!("heading={:.*}", 1, heading);

        thread::sleep(Duration::from_millis(100));
    }

}