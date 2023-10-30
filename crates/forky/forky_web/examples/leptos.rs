use leptos::*;

// fn main() {
//     mount_to_body(|| view! { <p>"Hello, world!"</p> })
// }

#[component]
fn App(
	cx: Scope,
	foo: u32,
	#[prop(default = false)] bar: bool,
) -> impl IntoView {
	let (count, set_count) = create_signal(cx, 0);

	view! {cx,
		<button
			on:click=move |_| {
				set_count(3);
			}
		>
			"Click me: "
			{move || count.get()}
		</button>
	}
}
