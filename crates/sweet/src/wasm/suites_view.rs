use super::*;
use crate::*;
use leptos::*;

#[component]
pub fn SuitesView<F>(cx: Scope, set_file: F) -> impl IntoView
where
	F: Fn(Option<String>) + 'static + Copy,
{
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
		<div
			class="sweet-suite"
			on:click= move|_|set_file(None)
			>"Home"</div>
		<br/>
		{suites.iter()
			.map(|suite|view!{cx,<SuiteView suite set_file/>})
			.collect::<Vec<_>>()
		}
	</div>
	}
}

#[component]
pub fn SuiteView<'a, F>(
	cx: Scope,
	set_file: F,
	suite: &'a TestSuite<TestCaseWasm>,
) -> impl IntoView
where
	F: Fn(Option<String>) + 'static,
{
	let file = suite.file.clone();
	let pretty = file
		.split("\\")
		.collect::<Vec<_>>()
		.iter()
		.rev()
		.take(3)
		.rev()
		.fold(String::new(), |mut acc, val| {
			acc.push_str(*val);
			acc.push_str(" > ");
			acc
		})
		.replace(".rs > ", "");

	// .collect::<Vec<_>>()


	// .join(" > ").replace(".rs", "");
	// let href = format!("?file={}", file);
	view! {cx,
	<div class="sweet-suite"
		on:click=move|_|set_file(Some(file.clone()))
	>
		{pretty}
	</div>
	}
}
