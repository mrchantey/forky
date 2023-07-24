use super::*;
use leptos::*;
use web_sys::UrlSearchParams;

#[component]
pub fn RunnerContainer(
	cx: Scope,
	#[prop(into)] file: Signal<Option<String>>,
) -> impl IntoView {
	move || {
		if let Some(file) = file() {
			let (loaded, set_loaded) = create_signal(cx, false);

			let class = move || {
				if loaded() {
					"full-size"
				} else {
					"full-size hidden"
				}
			};

			let params = UrlSearchParams::new().unwrap();
			params.set("run", "1");
			params.set("file", &file);
			let mut params = params.to_string().as_string().unwrap();
			params.insert_str(0, "?");
			view!(cx,
				<iframe
				frameBorder="0"
				class={class}
				on:load= move |_| {set_loaded(true)}
				src=params/>
			)
			.into_any()
		} else {
			view!(cx,
				<div class = "center-parent">
					<h2>"🤘 sweet as! 🤘"</h2>
				</div>
			)
			.into_any()
		}
	}
}

#[component]
pub fn Runner(_cx: Scope) -> impl IntoView {
	run_tests_wasm();
	""
}
