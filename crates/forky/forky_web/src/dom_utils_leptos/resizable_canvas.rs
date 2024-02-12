use crate::ResizeListener;
use leptos::html::Canvas;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::ResizeObserverSize;


pub fn resizable_canvas(el: NodeRef<Canvas>) {
	let listener = create_effect(move |_| {
		if let Some(canvas) = el() {
			let listener = ResizeListener::new(&canvas.clone(), move |val| {
				let first = val.device_pixel_content_box_size().get(0);
				let first = first.unchecked_ref::<ResizeObserverSize>();
				let width = first.inline_size();
				let height = first.block_size();
				canvas.set_width(width as u32);
				canvas.set_height(height as u32);
			});
			Some(listener)
		} else {
			None
		}
	});

	on_cleanup(move || drop(listener));
}
