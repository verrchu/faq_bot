use chrono::{DateTime, Utc};
use std::time::{Duration, UNIX_EPOCH};

pub fn hash(input: &str) -> String {
    format!("{:x}", md5::compute(input))
}

pub fn unixtime_to_datetime(timestamp: u32) -> String {
    let secs = UNIX_EPOCH + Duration::from_secs(u64::from(timestamp));

    DateTime::<Utc>::from(secs).format("%Y/%m/%d").to_string()
}
