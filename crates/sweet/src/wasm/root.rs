use super::*;
use forky_web::*;
use leptos::*;

pub fn interactive_mode() -> bool { !SearchParams::get_flag("run") }

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

	if interactive_mode() {
		view! {cx,
		<div class=sweet_style::SWEET_ROOT>
			<SuitesView set_file/>
			<RunnerContainer file/>
		</div>
		}
		.into_view(cx)
	} else {
		view! {cx, <Runner/>}.into_view(cx)
	}
}
