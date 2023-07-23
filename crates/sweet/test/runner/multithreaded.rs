use std::time::Duration;
use sweet::*;

sweet! {
	//should take 1 second
	test skip "thread 1" {
		std::thread::sleep(Duration::from_secs(1));
	}
	test skip "thread 2" {
		std::thread::sleep(Duration::from_secs(1));
	}

	// test "foobar"{
	// 	expect(true).to_be_false()?;
	// }
}
