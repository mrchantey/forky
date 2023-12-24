use crate::ResizeListener;
use leptos::html::Canvas;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::ResizeObserverSize;

#[component]
pub fn BevyCanvas(
	#[prop(optional)] canvas_ref: Option<NodeRef<Canvas>>,
) -> impl IntoView {
	let canvas_ref = if let Some(canvas_ref) = canvas_ref {
		canvas_ref
	} else {
		create_node_ref()
	};

	let listener = create_effect(move |_| {
		if let Some(canvas) = canvas_ref() {
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

	view! {<canvas
			class="bevy-canvas"
			data-bevy="primary_window"
			node_ref = canvas_ref
			/>
	}
}
