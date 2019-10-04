use std::time::{SystemTime, UNIX_EPOCH};

use crate::precision::Real;

#[derive(Debug)]
pub struct Timing {
    frame_number: u64,
    last_frame_timestamp: u64,
    last_frame_duration: u64,
    is_paused: bool,
    average_frame_duration: f64,
    fps: f64
}

impl Timing {
    pub fn update(&mut self) {
        if !self.is_paused {
            self.frame_number += 1;
        }
        let current_timestamp = Timing::get_time();
        self.last_frame_duration = current_timestamp - self.last_frame_timestamp;
        self.last_frame_timestamp = current_timestamp;
        if self.frame_number > 1 {
            if self.average_frame_duration <= 0.0 {
                self.average_frame_duration = self.last_frame_duration as f64;
            } else {
                self.average_frame_duration *= 0.99;
                self.average_frame_duration += 0.01 * self.last_frame_duration as f64;
                self.fps = 1000.0 / self.average_frame_duration;
            }
        }
    }
    pub fn get_duration(&self) -> Real {
        self.last_frame_duration as Real * 0.001
    }
    fn get_time() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards").as_millis() as u64
    }
}

impl Default for Timing {
    fn default() -> Timing {
        Timing {
            frame_number: 0,
            last_frame_timestamp: Timing::get_time(),
            last_frame_duration: 0,
            is_paused: false,
            average_frame_duration: 0.0,
            fps: 0.0
        }
    }
}