use super::*;
use crate::*;
use leptos::*;

// pub type SuitesVec = ;

#[component]
pub fn SuitesView<F>(
	cx: Scope,
	// #[prop(into)] _foo: Signal<u32>,
	suites: F,
) -> impl IntoView
where
	F: Fn() -> Vec<TestSuite<TestCaseWasm>>,
{
	view! {cx,
	<div>
	<h3>"Suites"</h3>
		{suites().iter()
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
	let href = format!("?file={}", file);
	view! {cx,
	<a
		href=href
	>
		{file}
	</a>
	}
}
