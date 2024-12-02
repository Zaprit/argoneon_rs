use std::{thread::sleep, time::Duration};

use rppal::i2c::{Error, I2c};

use crate::config;

pub fn set_fan_speed(speed: u8) -> Result<(), Error> {
    let mut i2c = I2c::new()?;

    // Set the I2C slave address to the device we're communicating with.
    i2c.set_slave_address(config::FAN_ADDR)?;

    if speed > 0 {
        // Spin up to prevent issues on older units
        i2c.smbus_send_byte(100)?;
        sleep(Duration::new(1, 0));
    }

    i2c.smbus_send_byte(speed)?;
    println!("writing to fan port, speed {}", speed);

    return Ok(());
}
