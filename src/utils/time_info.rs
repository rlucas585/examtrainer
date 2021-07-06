use std::time::Instant;

pub struct TimeInfo {
    pub start: Instant,
    pub end_time: chrono::DateTime<chrono::Local>,
}

impl TimeInfo {
    pub fn new(start: Instant, end_time: chrono::DateTime<chrono::Local>) -> Self {
        Self { start, end_time }
    }
}

