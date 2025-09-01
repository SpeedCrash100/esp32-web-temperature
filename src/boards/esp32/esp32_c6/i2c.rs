use embassy_executor::Spawner;
use esp_hal::{i2c::master::Config, Async};

type RawI2c = esp_hal::i2c::master::I2c<'static, Async>;
pub type I2c = crate::drivers::i2c::ector::EctorI2c;

pub async fn init_i2c(
    i2c: esp_hal::peripherals::I2C0<'static>,
    scl: esp_hal::peripherals::GPIO7<'static>,
    sda: esp_hal::peripherals::GPIO6<'static>,
    spawner: Spawner,
) -> I2c {
    let i2c = esp_hal::i2c::master::I2c::new(i2c, Config::default())
        .unwrap()
        .with_scl(scl)
        .with_sda(sda)
        .into_async();

    let addr = ector::actor!(
        spawner,
        i2c0_ector_task,
        crate::drivers::i2c::ector::I2cActor<RawI2c>,
        crate::drivers::i2c::ector::I2cActor { i2c }
    );

    let driver = I2c {
        address: addr.into(),
    };

    driver
}
