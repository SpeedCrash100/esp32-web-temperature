use crate::sensor_data::{filter::NoopFilter, SensorDataStore};

pub const SENSOR_STORE_CAP: usize = 32;

pub type TemperatureSensorStore = SensorDataStore<f32, NoopFilter<f32>, SENSOR_STORE_CAP>;

pub type HumiditySensorStore = SensorDataStore<f32, NoopFilter<f32>, SENSOR_STORE_CAP>;
