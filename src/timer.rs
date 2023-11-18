use std::time::{Instant, Duration};

pub struct Timer {
    last: Instant,
    delta_time: Duration,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            last: Instant::now(),
            delta_time: Duration::new(0, 0),
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last;
        self.last = now;
    }

    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }
}
