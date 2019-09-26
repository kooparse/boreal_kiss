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
