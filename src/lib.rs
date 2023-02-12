use std::time::{Duration, Instant};

pub struct Timer {
    start_time: Instant,
    minutes_to_time: u64,
}

impl Timer {
    pub fn new(minutes_to_time: u64) -> Timer {
        let start_time = Instant::now();
        Timer {
            start_time,
            minutes_to_time,
        }
    }
    pub fn get_elapsed_time(&self) -> Duration {
        let now = Instant::now();
        now.duration_since(self.start_time)
    }
    pub fn get_elapsed_minutes(&self) -> u64 {
        let elapsed_time = self.get_elapsed_time();
        elapsed_time.as_secs() / 60
    }
    pub fn get_minutes_remaining(&self) -> u64 {
        self.minutes_to_time - self.get_elapsed_minutes()
    }
}
