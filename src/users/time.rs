use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_time() -> u128 {
    let start = SystemTime::now();

    start.duration_since(UNIX_EPOCH).unwrap().as_millis()
}

pub fn get_expire_time() -> u128 {
    get_current_time() + 86400 * 3
}
