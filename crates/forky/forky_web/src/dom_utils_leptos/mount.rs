use crate::*;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::Document;

pub fn mount<F, N>(f: F)
where
	F: Fn() -> N + 'static,
	N: IntoView,
{
	set_panic_hook();
	leptos::mount_to_body(f)
}

pub fn mount_to_head<F, N>(f: F)
where
	F: Fn() -> N + 'static,
	N: IntoView,
{
	set_panic_hook();
	leptos::mount_to(Document::x_head().dyn_into().unwrap(), f);
}


pub fn mount_with_unmount<F, N>(f: F) -> Mount
where
	F: Fn() -> N + 'static,
	N: IntoView,
{
	mount(f);
	Mount
}

pub struct Mount;

impl Drop for Mount {
	fn drop(&mut self) { Document::x_clear() }
}



pub fn is_mounted() -> impl Fn() -> bool {
	let (mounted, _) = create_signal(());
	move || -> bool { mounted.try_get_untracked().is_some() }
}

pub async fn loop_while_mounted() {
	let mounted = is_mounted();
	while mounted() {
		wait_for_16_millis().await;
	}
}
