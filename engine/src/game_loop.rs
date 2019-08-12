use crate::time::Time;
use std::time::Duration;

// 16.6ms per frame for 60 frames per second.
const FPS: u32 = 60;

#[derive(Default)]
pub struct GameLoop {
    frame_rate: Duration,
    frame_number: u64,
    should_close: bool,
    start: Duration,
    time: Time,
}

impl GameLoop {
    pub fn new() -> Self {
        Self {
            frame_rate: Duration::from_secs(1) / FPS,
            ..Self::default()
        }
    }
    /// Start the game loop.
    pub fn start(&mut self, mut tick: impl FnMut(&Time, f64) -> bool) {
        self.should_close = false;
        self.time.now = Time::now();
        self.start = Time::now();

        while !self.should_close {
            self.update_time();

            let fps = (self.frame_number as f64)
                / Time::duration_to_secs(self.time.now - self.start);

            self.should_close = tick(&self.time, fps);

            self.frame_number += 1;
            self.sync_loop();
        }
    }
    /// Synchronize ticks to draw stuff at 60FPS.
    ///
    /// This function will sleep the main thread only if the current
    /// tick took less than 16.6ms to complete. If not, do nothing (yet).
    fn sync_loop(&mut self) {
        // This will set `None` if the computed duration is negative.
        let sleep_time =
            (self.time.now + self.frame_rate).checked_sub(Time::now());

        if let Some(sleep_time) = sleep_time {
            std::thread::sleep(sleep_time);
        } else {
            // unimplemented!();
        }
    }

    fn update_time(&mut self) {
        self.time.last_time = self.time.now;
        self.time.now = Time::now();
        self.time.dt =
            Time::duration_to_secs(self.time.now - self.time.last_time);
    }
}
