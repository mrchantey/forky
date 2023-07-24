use super::*;
use crate::*;
use forky_core::wasm::*;
use leptos::*;
use std::time::Duration;

fn run(collector: &TestCollectorWasm, config: &TestRunnerConfig) {
	let intro = TestRunner::pretty_print_intro(config);
	log!("{intro}");


	let start_time = performance_now();
	let results = collector.run(&config);
	let duration =
		Duration::from_millis((performance_now() - start_time) as u64);
	let summary = TestRunner::pretty_print_summary(&results, duration);

	log!("{summary}");
}

#[component]
pub fn Root(cx: Scope) -> impl IntoView {
	let config = TestRunnerConfig::from_search_params();
	let (collector, _) = create_signal(cx, TestCollectorWasm::new());

	let suites = move || {
		collector
			.get()
			.suites_to_run(&config)
			.iter()
			.map(|s| (*s).clone())
			.collect::<Vec<_>>()
	};

	// let foo = move || bar();

	// let col = collector.get();
	// let suites = col.suites_to_run(&config);


	view! {cx,
	<div>
		// <SuitesView _foo=foo/>
		<SuitesView suites=suites/>
	</div>
	}
}
