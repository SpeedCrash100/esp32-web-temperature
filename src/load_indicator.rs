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

                self.load.store(load as u8, Ordering::SeqCst);
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
