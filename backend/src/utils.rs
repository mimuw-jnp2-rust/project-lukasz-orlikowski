use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time() -> i32 {
    let time = SystemTime::now();
    let since_the_epoch = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    i32::try_from(since_the_epoch.as_secs()).ok().unwrap()
}