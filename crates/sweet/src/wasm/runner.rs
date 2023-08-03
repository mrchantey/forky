use super::settings::*;
use forky_web::*;
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
			let dark_iframe = SearchParams::get_flag(DARK_IFRAME_KEY);

			//avoid load flash of iframe
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
			//iframe body is inheriting #000 not sure why, below will force default white but result in flash
			//TODO this is all very messy, will need rework when doing proper light/dark mode
			let style = if dark_iframe {
				"background: #000000;"
			} else {
				"background: #FFFFFF;"
			};
			view!(cx,
				<div class="full-size" style=style>
				<iframe
				style=style
				class=class
				src=params
				frameBorder="0"
				on:load= move |_| {set_loaded(true)}
				/>
				</div>
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
