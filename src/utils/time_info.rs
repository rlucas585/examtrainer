use std::time::Instant;

pub struct TimeInfo {
    pub start: Instant,
    pub end_time: chrono::DateTime<chrono::Local>,
    pub end_instant: Instant,
}

impl TimeInfo {
    pub fn new(
        start: Instant,
        end_time: chrono::DateTime<chrono::Local>,
        end_instant: Instant,
    ) -> Self {
        Self {
            start,
            end_time,
            end_instant,
        }
    }
}
