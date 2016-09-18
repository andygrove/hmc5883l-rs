use std::f32;
use std::io::Cursor;
use std::thread;
use std::time::Duration;

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

        // set gain
        try!(dev.smbus_write_byte_data(0x01, 0x20));
        thread::sleep(Duration::from_millis(100));

        // set in continuous mode
        try!(dev.smbus_write_byte_data(0x02, 0x00));
        thread::sleep(Duration::from_millis(100));

        Ok(HMC5883L { dev: Box::new(dev) })
    }

    pub fn read(&mut self) -> Result<(f32, f32, f32), Box<LinuxI2CError>> {

        // start reading from register 03 (x)
        try!(self.dev.smbus_write_byte(0x03));
        thread::sleep(Duration::from_millis(100));

        // read two bytes each from registers 03 through 05 (x, y, z)
        let mut buf: [u8; 6] = [0; 6];
        try!(self.dev.read(&mut buf));
        println!("{:?}", buf);


        let x : i16 = ((buf[0] as i16) << 8) as i16 | buf[1] as i16;
        let y : i16 = ((buf[2] as i16) << 8) as i16 | buf[3] as i16;
        let z : i16 = ((buf[4] as i16) << 8) as i16 | buf[5] as i16;

        // parse the values
//        let mut rdr = Cursor::new(&buf);
//        let x = rdr.read_i16::<BigEndian>().unwrap() as f32;
//        let y = rdr.read_i16::<BigEndian>().unwrap() as f32;
//        let z = rdr.read_i16::<BigEndian>().unwrap() as f32;

        // return tuple containing x, y, z values
        Ok((x as f32, y as f32, z as f32))
    }

}



