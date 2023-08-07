use super::settings::Settings;
use super::*;
use crate::MATCHES_KEY;
use forky_web::*;
use leptos::*;

// pub fn get_file() -> Option<String> {  }

pub fn suite_matches_none(matches: &Vec<String>) -> bool {
	matches.len() == 1 && matches[0] == "!"
}

#[component]
pub fn Root(cx: Scope) -> impl IntoView {
	let (suite_matches, set_matches) =
		create_signal(cx, SearchParams::get_all(MATCHES_KEY));
	create_effect(cx, move |_| {
		let suite_matches = suite_matches();
		History::remove_param(MATCHES_KEY);
		for suite_match in suite_matches.iter() {
			History::append_param(MATCHES_KEY, suite_match);
		}
	});

	view! {cx,
		<div class=sweet_style::SWEET_ROOT>
			<div class=sweet_style::SWEET_CONTENTS>
				<Settings/>
				<SuitesView set_matches/>
			</div>
			<RunnerContainer suite_matches/>
		</div>
	}
}
