use std::thread::sleep;
use std::time::Duration;

pub fn set_timeout(ms: u64) {
    sleep(Duration::from_millis(ms));
}
