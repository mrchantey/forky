use leptos::*;

#[component]
pub fn Button(cx: Scope, children: Children) -> impl IntoView {
	view! {cx,
		<button>{children(cx)}</button>
	}
}
