extern crate hmc5883l;
use hmc5883l::*;

use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

/// based on https://github.com/adafruit/Adafruit_HMC5883_Unified/blob/master/examples/magsensor/magsensor.ino
fn main() {

    let mut mag = HMC5883L::new("/dev/i2c-1", 0x1E).unwrap();

    let gauss_lsb_xy = 1100.0;
    let gauss_lsb_z  =  980.0;
    let declination_angle = 0.22; // you need to set this based on your location

    loop {

        //TODO: this should be (x, y, z) but for some reason I seem to be getting them
        // in this order
        let (x, z, y) = mag.read().unwrap();

        // convert to micro-teslas
        let (x, y, z) = (x/gauss_lsb_xy*100.0, y/gauss_lsb_xy*100.0, z/gauss_lsb_z*100.0);


        // testing
        //let (x, y, z) = (38.18_f32, -13.18_f32, -15.10_f32); // maps to 353 degrees heading
     
        let mut heading = y.atan2(x) + declination_angle;

        if heading < 0.0 {
            heading += 2.0 * PI;
        }

        if heading > 2.0 * PI {
            heading -= 2.0 * PI;
        }

        // Convert radians to degrees for readability.
        heading = heading * 180.0 / PI;

        println!("x={:.*}, y={:.*}, z={:.*} uT: heading={:.*}", 1, x, 1, y, 1, z, 1, heading);

        thread::sleep(Duration::from_millis(500));
    }

}
