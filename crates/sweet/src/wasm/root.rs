use super::*;
use forky_core::wasm::*;
use leptos::*;

pub fn interactive_mode() -> bool { !SearchParams::get_flag("run") }

#[component]
pub fn Root(cx: Scope) -> impl IntoView {
	if interactive_mode() {
		view! {cx,
		<div class="sweet-root">
			<SuitesView/>
			<RunnerContainer/>
		</div>
		}
		.into_view(cx)
	} else {
		view! {cx, <Runner/>}.into_view(cx)
	}
}
