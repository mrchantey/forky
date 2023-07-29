use super::*;
use leptos::*;

pub fn mount<F, N>(f: F)
where
	F: FnOnce(Scope) -> N + 'static,
	N: IntoView,
{
	set_panic_hook();
	leptos::mount_to_body(f)
}
