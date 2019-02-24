use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Time {
    pub delta_time: f64,
    pub delta_duration: Duration,
    pub start_time: Instant,
}

impl Time {
    #[inline]
    pub fn new() -> Self {
        Time {
            delta_time: 0.0,
            delta_duration: Duration::new(0, 0),
            start_time: Instant::now(),
        }
    }

    #[inline]
    pub fn set_delta(&mut self, delta: Duration) -> f64 {
        self.delta_time =
            (delta.as_secs() as f64) + (delta.subsec_nanos() as f64) / 1_000_000_000.0;
        self.delta_duration = delta;
        self.delta_time
    }
}
