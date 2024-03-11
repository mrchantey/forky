use crate::test_runner::*;
use crate::wasm::*;
use leptos::html::Iframe;
use leptos::*;
use web_sys::HtmlIFrameElement;


fn suite_matches_none(matches: &Vec<String>) -> bool {
	matches.len() == 0 || (matches.len() == 1 && matches[0] == "!")
}

#[component]
pub fn RunnerContainer(
	#[prop(into)] suite_matches: Signal<Vec<String>>,
) -> impl IntoView {
	// let file_unwrapped = move || file().unwrap();
	move || {
		let suite_matches = suite_matches();
		if suite_matches_none(&suite_matches) {
			view! {
				<div class="center-parent">
					<h2>"🤘 sweet as! 🤘"</h2>
				</div>
			}
			.into_view()
		} else {
			view! { <Runner _suite_matches=suite_matches/> }
				// view! {<RunnerContainerActual file=Signal::derive(file_unwrapped)/>}
				.into_view()
		}
	}
}


#[component]
pub fn Runner(
	#[prop(into)] _suite_matches: Vec<String>,
	// #[prop(into)] file: Signal<String>,
) -> impl IntoView {
	let (loaded, set_loaded) = create_signal(false);

	let iframe: NodeRef<Iframe> = create_node_ref();

	create_effect(move |_| {
		if let Some(iframe) = iframe() {
			let iframe: &HtmlIFrameElement = &iframe;
			let config = TestRunnerConfig::from_search_params();
			TestRunnerWasm::run(&config, iframe.clone())
		}
	});



	//avoid load flash of iframe
	let class = move || {
		if loaded() {
			"full-size"
		} else {
			"full-size hidden"
		}
	};
	// is this already set in `sweet_style.css?`
	let style = "background: transparent;";

	view! {
		<div class="full-size" style=style>
			<iframe
				allow-same-origin
				// allow="browsing-topics"
				style=style
				class=class
				// src=url
				node_ref=iframe
				frameBorder="0"
				on:load=move |_| { set_loaded(true) }
			></iframe>
		</div>
	}
}
