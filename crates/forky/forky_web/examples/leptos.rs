// use forky_web::*;
use leptos::*;

fn main() {
	mount_to_body(|| {
		view! {
			<div>
				<App/>
			</div>
		}
	})
}

#[component]
fn App(
	// foo: u32,
	#[prop(default = false)] _bar: bool,
) -> impl IntoView {
	let (count, set_count) = create_signal(0);

	view! {
		<button on:click=move |_| {
			set_count(count.get() + 1);
		}>

			"Click me: " {move || count.get()}
		</button>
	}
}
