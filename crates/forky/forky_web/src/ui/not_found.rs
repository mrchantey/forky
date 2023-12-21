use crate::*;
use leptos::*;
use std::time::Duration;

#[component]
pub fn NotFound(duration: u8) -> impl IntoView {
	let (count, set_count) = create_signal(duration);
	set_interval(
		move || set_count.update(|c| *c -= 1),
		Duration::from_secs(1),
	);
	set_timeout(
		move || Location::navigate("/"),
		Duration::from_secs(count() as u64),
	);
	view! {
	<div>
		{format!("Page Not Found: {}", path_name())}
		<br/>
		{format!("Redirecting in ")}{count}
	</div>
	}
}
