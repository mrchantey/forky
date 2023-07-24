use super::*;
use crate::*;
use leptos::*;
use web_sys::UrlSearchParams;

#[component]
pub fn RunnerContainer(cx: Scope) -> impl IntoView {
	let config = TestRunnerConfig::from_search_params();
	if config.files.len() != 1 {
		view!(cx, <h2>"Select a suite to run..."</h2>).into_any()
	} else {
		let params = UrlSearchParams::new().unwrap();
		params.set("run", "1");
		params.set("file", config.files[0].as_str());
		// run_tests_wasm(&collector, &config);
		let mut params = params.to_string().as_string().unwrap();
		params.insert_str(0, "?");
		view!(cx,
			<iframe
			frameBorder="0"
			class="full-size"
			src=params/>
		)
		.into_any()
	}
}

#[component]
pub fn Runner(cx: Scope) -> impl IntoView {
	let config = TestRunnerConfig::from_search_params();
	let collector = TestCollectorWasm::new();
	run_tests_wasm(&collector, &config);
	""
}
