use std::time::{Instant, Duration};


pub struct Timer {
    clock: Option<Instant>
}

impl Timer {
    pub fn new() -> Self {
        Timer { clock: Some(Instant::now()) }
    }

    pub fn start(&mut self) {
        self.clock = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.clock = None;
    }

    pub fn elapsed(&self) -> Duration {
        self.clock.unwrap().elapsed()
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.clock.unwrap().duration_since(earlier)
    }
}

