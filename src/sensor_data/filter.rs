mod noop;

use embassy_time::Duration;

use crate::sensor_data::TimedSensorData;

pub use noop::NoopFilter;

pub trait Filter {
    type Item;

    /// Filter value using previous stored value and current one with times attached
    ///
    /// # Arguments
    /// - `previous` - previous sensor data within window_size
    /// - `data` - new data
    /// - `window_size` - the target filter window size
    ///
    /// # Returns
    /// - None, if value should be ignored
    /// - Some(item), if current value should be replaced with new one
    fn filter(
        &mut self,
        previous: &TimedSensorData<Self::Item>,
        data: &TimedSensorData<Self::Item>,
        window_size: Duration,
    ) -> Option<Self::Item>;
}
