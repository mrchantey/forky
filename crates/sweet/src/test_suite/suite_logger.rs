pub trait SuiteLogger
where
	Self: Sized,
{
	fn on_start(start_str: String) -> Self;
	fn on_end(self, end_str: String);
}

// fn get_now() -> Duration {
// 	#[cfg(target_arch = "wasm32")]
// 	return Duration::from_millis(
// 		web_sys::window().unwrap().performance().unwrap().now() as u64,
// 	);
// 	#[cfg(not(target_arch = "wasm32"))]
// 	return std::time::Instant::now().elapsed(); //TODO now().elapsed is incorrect
// }
