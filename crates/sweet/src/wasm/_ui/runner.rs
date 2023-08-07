use super::settings::*;
use crate::{MATCHES_KEY, suite_matches_none};
use forky_web::*;
use leptos::html::Iframe;
use leptos::*;
use web_sys::UrlSearchParams;

#[component]
pub fn RunnerContainer(
	cx: Scope,
	#[prop(into)] suite_matches: Signal<Vec<String>>,
) -> impl IntoView {
	// let file_unwrapped = move || file().unwrap();
	move || {
		let suite_matches = suite_matches();
		if suite_matches_none(&suite_matches) {
			view!(cx,
				<div class = "center-parent">
					<h2>"🤘 sweet as! 🤘"</h2>
				</div>
			)
			.into_view(cx)
		} else {
			view! {cx,<RunnerContainerActual suite_matches/>}
				// view! {cx,<RunnerContainerActual file=Signal::derive(cx, file_unwrapped)/>}
				.into_view(cx)
		}
	}
}


#[component]
pub fn RunnerContainerActual(
	cx: Scope,
	#[prop(into)] suite_matches: Vec<String>,
	// #[prop(into)] file: Signal<String>,
) -> impl IntoView {
	let (loaded, set_loaded) = create_signal(cx, false);
	let dark_iframe = SearchParams::get_flag(DARK_IFRAME_KEY);

	let iframe: NodeRef<Iframe> = create_node_ref(cx);

	//avoid load flash of iframe
	let class = move || {
		if loaded() {
			"full-size"
		} else {
			"full-size hidden"
		}
	};
	//iframe body is inheriting #000 not sure why, below will force default white but result in flash
	//TODO this is all very messy, will need rework when doing proper light/dark mode
	let style = if dark_iframe {
		"background: #000000;"
	} else {
		"background: #FFFFFF;"
	};

	let url = move || {
		// let file = file();
		let params = UrlSearchParams::new().unwrap();
		params.set("run", "1");
		for matcher in suite_matches.iter() {
			params.append(MATCHES_KEY, matcher);
		}
		let mut params = params.to_string().as_string().unwrap();
		params.insert_str(0, "?");
		params
	};


	let iframe_view = move || {
		let url = url();
		view!(cx,<iframe
			style=style
			class=class
			src=url
			node_ref=iframe
			frameBorder="0"
			on:load= move |_| {set_loaded(true)}
			/>)
	};

	view!(cx,
		<div class="full-size" style=style>
			{iframe_view}
		</div>
	)
}
