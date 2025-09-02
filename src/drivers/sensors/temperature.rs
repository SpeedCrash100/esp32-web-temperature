use core::future::Future;

pub trait TemperatureSensorAsync {
    type ReadTemp;

    fn read_temperature(&mut self) -> impl Future<Output = Self::ReadTemp>;
}
