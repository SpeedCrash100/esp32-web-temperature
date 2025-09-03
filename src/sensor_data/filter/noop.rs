use core::marker::PhantomData;

use super::Filter;

/// Filter that does not filter at all
#[derive(Debug, Default)]
pub struct NoopFilter<T>(PhantomData<T>);

impl<T> Filter for NoopFilter<T>
where
    T: Copy,
{
    type Item = T;
    fn filter(
        &mut self,
        _: &crate::sensor_data::TimedSensorData<Self::Item>,
        data: &crate::sensor_data::TimedSensorData<Self::Item>,
        _: embassy_time::Duration,
    ) -> Option<Self::Item> {
        Some(*data.get())
    }
}
