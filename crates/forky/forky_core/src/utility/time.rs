use std::thread;
use std::time;




pub fn sleep(secs: u64) { thread::sleep(time::Duration::from_secs(secs)); }
pub fn sleep_ms(millis: u64) {
	thread::sleep(time::Duration::from_millis(millis));
}
