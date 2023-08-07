use super::*;
use crate::*;
use forky_core::*;
use forky_web::forky_style;
use leptos::*;
use web_sys::console;

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
	<h3
		class=forky_style::BUTTON_LIKE
		on:click= move|_|{
			console::clear();
			set_file(None)
		}
	>"Suites"</h3>
	<br/>
	{suites.iter()
		.map(|suite|view!{cx,<SuiteView suite set_file/>})
		.collect::<Vec<_>>()
	}
	}
}

#[component]
pub fn SuiteView<'a, F>(
	cx: Scope,
	set_file: F,
	suite: &'a TestSuiteWasm,
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
	view! {cx,
	<div class=spacecat!(forky_style::BUTTON_LIKE,sweet_style::SWEET_SUITE)
		on:click=move|_|set_file(Some(file.clone()))
	>
		{pretty}
	</div>
	}
}
