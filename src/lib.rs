use std::f32;
use std::io::Cursor;

extern crate byteorder;
use self::byteorder::{BigEndian, ReadBytesExt};

extern crate i2cdev;
use self::i2cdev::core::*;
use self::i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

pub struct HMC5883L {
    dev: Box<LinuxI2CDevice>
}

impl HMC5883L {

    pub fn new(filename: &'static str, address: u16) -> Result<Self, Box<LinuxI2CError>> {

        let mut dev = try!(LinuxI2CDevice::new(filename, address));

        // set in continuous mode
        try!(dev.smbus_write_byte_data(0x02, 0x00));
       
        // set gain 
        try!(dev.smbus_write_byte_data(0x01, 0x20));

        Ok(HMC5883L { dev: Box::new(dev) })
    }

    pub fn read(&mut self) -> Result<(f32, f32, f32), Box<LinuxI2CError>> {

        // start reading from register 03 (x)
        try!(self.dev.smbus_write_byte(0x03));

        // read two bytes each from registers 03 through 05 (x, y, z)
        let mut buf: [u8; 6] = [0; 6];
        try!(self.dev.read(&mut buf));

        // parse the values
        let mut rdr = Cursor::new(&buf);
        let x = rdr.read_i16::<BigEndian>().unwrap() as f32;
        let y = rdr.read_i16::<BigEndian>().unwrap() as f32;
        let z = rdr.read_i16::<BigEndian>().unwrap() as f32;

        // return tuple containing x, y, z values
        Ok((x, y, z))
    }

}

fn calc_bearing() {
//    X: 38.18  Y: -13.18  Z: -15.10  uT
//    Heading (degrees): 353.56

}



