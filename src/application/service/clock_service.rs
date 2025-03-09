use chrono::{DateTime, Utc};

pub struct ClockService {}

impl ClockService {
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }
}