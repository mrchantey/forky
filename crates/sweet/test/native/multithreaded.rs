// use std::thread::sleep;
// use std::time::Duration;
// use sweet::*;

// const DELAY: u64 = 1;

// sweet! {
// 	//should take 1 second
// 	test "sync 1" {
// 		println!("sync 1");
// 		sleep(Duration::from_secs(DELAY));
// 	}
// 	test "sync 2" {
// 		println!("sync 2");
// 		sleep(Duration::from_secs(DELAY));
// 	}
// 	test "async 1"{
// 		async{}.await;
// 		println!("async 1");
// 		sleep(Duration::from_secs(DELAY));
// 	}
// 	test "async 2"{
// 		async{}.await;
// 		println!("async 2");
// 		sleep(Duration::from_secs(DELAY));
// 	}
// 	test skip non_send "nonsend 1"{
// 		async{}.await;
// 		println!("nonsend 1");
// 	}
// }
