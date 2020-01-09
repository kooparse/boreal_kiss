use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Default)]
pub struct Time {
    pub dt: f64,
    pub now: Duration,
    pub last_time: Duration,
}

impl Time {
    pub fn now() -> Duration {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards.")
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Timer {
    threshold: f64,
    elapsed: f64, 
    init_called: bool,
}

impl Timer {
    pub fn new(threshold: f64) -> Self {
        Self {
            threshold,
            ..Self::default()
        }
    }

    pub fn is_passed(&mut self, dt: f64, delayed: f64) -> bool {
        self.elapsed += dt;

        if !self.init_called && self.elapsed >= delayed {
            self.init_called  = true;
            return true;
        }

        if self.elapsed >= self.threshold {
            self.elapsed = 0.;
            return true;
        }

        false
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            threshold: 1.,
            elapsed: 0.,
            init_called: false
        }
    }
}
