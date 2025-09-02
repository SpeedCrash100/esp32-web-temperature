use core::sync::atomic::{AtomicU8, Ordering};

use esp_hal::time::{Duration, Instant};

pub struct LoadExecutorHook {
    poll_time: Option<Instant>,
    idle_time: Option<Instant>,
    load: &'static AtomicU8,
}

impl LoadExecutorHook {
    pub fn new(load: &'static AtomicU8) -> Self {
        Self {
            poll_time: None,
            idle_time: None,
            load,
        }
    }

    fn update_load(&self, current_load: u8, time: Duration) {
        let prev_load = self.load.load(Ordering::SeqCst) as u64;
        let filter_coff = (time.as_micros()).clamp(10, 1000) / 10; // This will make 0-100 % we will use from new value if peaked just once

        let oldest_factor = 100 - filter_coff;
        let newest_factor = filter_coff;

        let oldest = oldest_factor * prev_load / 100;
        let newest = newest_factor * (current_load as u64) / 100;

        let load = (oldest + newest).clamp(0, 100) as u8;

        self.load.store(load, Ordering::SeqCst);
    }
}

impl esp_hal_embassy::Callbacks for LoadExecutorHook {
    fn before_poll(&mut self) {
        let now = Instant::now();

        if let Some(stored_poll_time) = self.poll_time.clone() {
            // We'd take idle time to avoid multiple computations with same idle
            if let Some(stored_idle_time) = self.idle_time.take() {
                let full_time = now - stored_poll_time;
                let load_time = stored_idle_time - stored_poll_time;

                let load = if full_time != Duration::ZERO {
                    load_time.as_micros() * 100 / full_time.as_micros()
                } else {
                    100
                };

                self.update_load(load as u8, full_time);
            } else {
                // Idle time not stored. Do nothing
            }
        } else {
            // No Poll time, just store it
        }

        self.poll_time = Some(now)
    }

    fn on_idle(&mut self) {
        self.idle_time = Some(Instant::now());
    }
}
