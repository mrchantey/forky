//

use std::time::Duration;


pub fn sleep_ms(millis: u64) {
	std::thread::sleep(Duration::from_millis(millis));
}
pub fn sleep_forever() -> ! {
	loop {
		sleep_ms(16); //60fps
	}
}
