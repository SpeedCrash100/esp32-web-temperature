use crate::drivers::sensors::temperature::TemperatureSensorAsync;
use defmt::error;

pub struct Lm75B<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> Lm75B<I2C> {
    pub fn new(i2c: I2C, address: u8) -> Self {
        Lm75B { i2c, address }
    }
}

impl<I2C> TemperatureSensorAsync for Lm75B<I2C>
where
    I2C: embedded_hal_async::i2c::I2c,
{
    type ReadTemp = f32;
    async fn read_temperature(&mut self) -> Self::ReadTemp {
        let mut data = [0_u8; 2];
        // 0x00 - Temperature register
        if let Err(_) = self.i2c.write_read(self.address, &[0x00], &mut data).await {
            error!("lm75b: read_temperature failed");
        }

        let msb = data[0];
        let lsb = data[1];

        let msb = f32::from(msb as i8);
        let decimal = f32::from((lsb & 0b1111_1000 as u8) >> 5) * 0.125;
        msb + decimal
    }
}
