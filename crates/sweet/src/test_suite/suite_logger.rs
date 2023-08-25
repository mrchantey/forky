use crate::TestSuiteResult;
use colorize::*;

pub trait SuiteLogger
where
	Self: Sized,
{
	fn on_start(start_str: String) -> Self;
	fn on_end(self, end_str: String);

	fn start(file: &str) -> Self { Self::on_start(Self::in_progress_str(file)) }

	fn end(self, file: &str, result: &TestSuiteResult, message: String) {
		let mut end_str = Self::end_str(file, result);
		end_str += &message;
		self.on_end(end_str);
	}

	fn in_progress_str(file: &str) -> String {
		let mut value = " RUNS ".black().bold().yellowb();
		value += " ";
		value += pretty_path(file).as_str();
		value
	}


	fn end_str(file: &str, result: &TestSuiteResult) -> String {
		let mut val = if result.failed == 0 {
			" PASS ".black().bold().greenb()
		} else {
			" FAIL ".black().bold().redb()
		};
		val += " ";
		val += pretty_path(file).as_str();
		val
	}
}

fn pretty_path(file: &str) -> String {
	let mut splt = file.split("\\").collect::<Vec<&str>>();
	let _file = splt.pop();
	let file = _file.unwrap_or_default().to_string().bold();
	let path = splt.join("\\").faint();
	let middle = "\\".to_string().faint();
	format!("{path}{middle}{file}")
}

// fn get_now() -> Duration {
// 	#[cfg(target_arch = "wasm32")]
// 	return Duration::from_millis(
// 		web_sys::window().unwrap().performance().unwrap().now() as u64,
// 	);
// 	#[cfg(not(target_arch = "wasm32"))]
// 	return std::time::Instant::now().elapsed(); //TODO now().elapsed is incorrect
// }
