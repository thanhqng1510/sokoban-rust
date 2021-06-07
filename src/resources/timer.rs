use std::time::{Instant, Duration};


pub struct Timer {
    clock: Instant
}

impl Timer {
    pub fn new() -> Self {
        Timer { clock: Instant::now() }
    }

    pub fn elapsed(&self) -> Duration {
        self.clock.elapsed()
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.clock.duration_since(earlier)
    }
}

