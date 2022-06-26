use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local};

pub fn get_time() -> i32 {
    let time = SystemTime::now();
    let since_the_epoch = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    i32::try_from(since_the_epoch.as_secs()).ok().unwrap()
}

pub fn get_date() -> String {
    let local: DateTime<Local> = Local::now();
    format!("{}", local)
}

pub fn matches(x: Option<i32>, y: i32) -> bool{
    x.is_some() && x.unwrap() == y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches()  {
        assert!(matches(Some(10), 10));
        assert!(!matches(Some(10), 11));
        assert!(!matches(None, 10));
    }
}