use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn system_time_to_epoch(time: SystemTime) -> Duration {
    return time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards :O");
}
