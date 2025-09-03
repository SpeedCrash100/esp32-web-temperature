//!
//! Storage to hold data with time attached
//!

pub mod filter;

use embassy_time::{Duration, Instant};
use ringbuffer::RingBuffer;

pub use filter::Filter;

/// Sensor data with time attached
pub struct TimedSensorData<T>(Instant, T);
impl<T> TimedSensorData<T> {
    pub fn get(&self) -> &T {
        &self.1
    }

    pub fn time(&self) -> &Instant {
        &self.0
    }
}

pub type Buffer<T, const N: usize> = ringbuffer::ConstGenericRingBuffer<TimedSensorData<T>, N>;

pub struct SensorDataStore<T, FILTER, const N: usize> {
    buffer: Buffer<T, N>,
    /// The time to average data
    /// If data added within this time after previous add, it will passed over smoothing filter
    /// When last cell, outside of this duration the data will be added as new cell in buffer
    window_size: Duration,
    filter: FILTER,
    last_pushed: Instant,
}

impl<T, FILTER, const N: usize> SensorDataStore<T, FILTER, N> {
    pub fn new(window_size: Duration, filter: FILTER) -> Self {
        assert!(0 < N);

        Self {
            buffer: Default::default(),
            window_size,
            last_pushed: Instant::now(),
            filter,
        }
    }
}

impl<T, FILTER, const N: usize> SensorDataStore<T, FILTER, N>
where
    T: Copy,
    FILTER: Filter<Item = T>,
{
    /// Adds sensor data to store with applying filter
    ///
    /// # Note
    /// It does not mean, that len will increase. The sensor data can be merged with latest value
    /// by apply
    pub fn add(&mut self, data: T) {
        let now = Instant::now();
        if self.is_empty() {
            self.buffer.push(TimedSensorData(now, data));
            self.last_pushed = now;
            return;
        }

        let last = self.last().unwrap(); // Checked before we're not empty
        let elapsed = now.duration_since(*last.time());

        // We insert last_pushed time instead of original so filter can reason about passed Instant
        let timed_prev = TimedSensorData(self.last_pushed, *last.get());
        let timed_new = TimedSensorData(now, data);
        let new_data = self
            .filter
            .filter(&timed_prev, &timed_new, self.window_size);

        let Some(new_data) = new_data else {
            // Filter dropped data -> ignore it
            return;
        };

        if self.window_size <= elapsed {
            // We outside of window size, need to add new element
            self.buffer.push(TimedSensorData(now, new_data));
        } else {
            let cell = self.buffer.back_mut().unwrap();
            cell.1 = new_data;
        }
    }

    /// Gets count of stored data
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Checks whatever store is impty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Gets last sensor reading available
    pub fn last(&self) -> Option<&TimedSensorData<T>> {
        self.buffer.back()
    }
}
