use super::*;
use crate::*;
use forky_core::*;
use forky_web::forky_style;
use leptos::*;
use web_sys::console;

#[component]
pub fn SuitesView(
	cx: Scope,
	#[prop(into)] set_matches: WriteSignal<Vec<String>>,
) -> impl IntoView {
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
			set_matches(vec!["!".to_string()])
		}
	>"Suites"</h3>
	<SuiteButton
		name="all".to_string()
		matcher="*".to_string()
		set_matches
	/>
	{suites.iter()
		.map(|suite|view!{cx,<SuiteView suite set_matches/>})
		.collect::<Vec<_>>()
	}
	}
}

#[component]
pub fn SuiteView<'a>(
	cx: Scope,
	#[prop(into)] set_matches: WriteSignal<Vec<String>>,
	suite: &'a TestSuiteWasm,
) -> impl IntoView {
	let file_str = suite.file.to_string_lossy().to_string();

	let pretty = suite
		.file
		.iter()
		.map(|p| p.to_string_lossy())
		.collect::<Vec<_>>()
		.iter()
		.rev()
		.take(3)
		.rev()
		.fold(String::new(), |mut acc, val| {
			acc.push_str(val);
			acc.push_str(" > ");
			acc
		})
		.replace(".rs > ", "");
	view! {cx,
	<div class=spacecat!(forky_style::BUTTON_LIKE,sweet_style::SWEET_SUITE)
		on:click=move|_|set_matches(vec![file_str.clone()])
	>
		{pretty}
	</div>
	}
}

#[component]
pub fn SuiteButton(
	cx: Scope,
	name: String,
	matcher: String,
	#[prop(into)] set_matches: WriteSignal<Vec<String>>,
) -> impl IntoView {
	view! {cx,
	<div class=spacecat!(forky_style::BUTTON_LIKE,sweet_style::SWEET_SUITE)
		on:click=move|_|set_matches(vec![matcher.clone()])
	>
		{name}
	</div>
	}
}
