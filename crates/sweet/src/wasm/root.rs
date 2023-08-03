use super::settings::Settings;
use super::*;
use forky_web::*;
use leptos::*;

pub fn get_file() -> Option<String> { SearchParams::get("file") }

#[component]
pub fn Root(cx: Scope) -> impl IntoView {
	let (file, set_file) = create_signal(cx, get_file());
	create_effect(cx, move |_| {
		if let Some(file) = file() {
			History::set_param("file", file.as_str());
		} else {
			History::remove_param("file");
		}
	});

	view! {cx,
		<div class=sweet_style::SWEET_ROOT>
			<div class=sweet_style::SWEET_CONTENTS>
				<Settings/>
				<SuitesView set_file/>
			</div>
			<RunnerContainer file/>
		</div>
	}
}
