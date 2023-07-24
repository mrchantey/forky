use super::*;
use crate::*;
use leptos::*;

#[component]
pub fn SuitesView(cx: Scope) -> impl IntoView {
	let config = TestRunnerConfig::default();
	let collector = TestCollectorWasm::new();

	let suites = collector
		.suites_to_run(&config)
		.iter()
		.map(|s| (*s).clone())
		.collect::<Vec<_>>();


	view! {cx,
	<div class="sweet-contents">
	<h3>"Suites"</h3>
		<a href="http://127.0.0.1:8080">"Home"</a>
		<br/>
		{suites.iter()
			.map(|suite|view!{cx,<SuiteView suite/>})
			.collect::<Vec<_>>()
		}
	</div>
	}
}

#[component]
pub fn SuiteView<'a>(
	cx: Scope,
	suite: &'a TestSuite<TestCaseWasm>,
) -> impl IntoView {
	let file = suite.file.clone();
	let pretty = file.replace("\\", " > ").replace(".rs", "");
	let href = format!("?file={}", file);
	view! {cx,
	<a
		href=href
	>
		{pretty}
	</a>
	}
}
