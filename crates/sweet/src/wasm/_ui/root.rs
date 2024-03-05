use super::settings::Settings;
use super::*;
use crate::wasm::MATCHES_KEY;
use forky_web::*;
use leptos::*;

#[component]
pub fn Root() -> impl IntoView {
	let (suite_matches, set_matches) =
		create_signal(SearchParams::get_all(MATCHES_KEY));
	create_effect(move |_| {
		let suite_matches = suite_matches();
		History::remove_param(MATCHES_KEY);
		for suite_match in suite_matches.iter() {
			History::append_param(MATCHES_KEY, suite_match);
		}
	});

	view! {
		<div class=sweet_style::SWEET_ROOT>
			<div class=sweet_style::SWEET_CONTENTS>
				<Settings/>
				<SuitesView set_matches/>
			</div>
			<RunnerContainer suite_matches/>
		</div>
	}
}
