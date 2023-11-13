use forky_web::*;
use leptos::*;

#[component]
pub fn Settings() -> impl IntoView {
	view! {
	<h3>"Settings"</h3>
	<DarkIframe/>
	}
}

pub const DARK_IFRAME_KEY: &str = "dark-iframe";

#[component]
pub fn DarkIframe() -> impl IntoView {
	let (checked, set_checked) =
		create_signal(SearchParams::get_flag(DARK_IFRAME_KEY));

	create_effect(move |_| {
		SearchParams::set_flag(DARK_IFRAME_KEY, checked());
	});

	view! {
	<div>
		<label for="dark-iframe-checkbox">"dark iframe"</label>
		<input
			id="dark-iframe-checkbox"
			type="checkbox"
			prop:checked=checked
			on:input={move |ev|{
				set_checked(event_target_checked(&ev));
			}}
			/>
	</div>
	}
}
