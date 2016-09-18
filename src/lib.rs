use std::time::Duration;
use std::f32;
use std::io::prelude::*;
use std::io::Cursor;

extern crate byteorder;
use self::byteorder::{BigEndian, ReadBytesExt};

extern crate i2cdev;
use self::i2cdev::core::*;
use self::i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

pub struct HMC5883L {
    dev: LinuxI2CDevice
}

impl HMC5883L {

    pub fn new(filename: &'static str, address: u8) -> Self {

        let mut dev = LinuxI2CDevice::new("/dev/i2c-1", 0x1E).unwrap();

        // set in continuous mode
        dev.smbus_write_byte_data(0x02, 0x00).unwrap();

        HMC5883L { dev: dev }
    }

    pub fn read(&mut self) -> (f32, f32, f32) {

        // start reading from register 03 (x)
        self.smbus_write_byte(0x03).unwrap();

        // read two bytes each from registers 03 through 05 (x, y, z)
        let mut buf: [u8; 6] = [0; 6];
        dev.read(&mut buf).unwrap();

        // parse the values
        let mut rdr = Cursor::new(&buf);
        let x = rdr.read_i16::<BigEndian>().unwrap() as f32;
        let y = rdr.read_i16::<BigEndian>().unwrap() as f32;
        let z = rdr.read_i16::<BigEndian>().unwrap() as f32;

        // return tuple containing x, y, z values
        (x, y, z)
    }

}
