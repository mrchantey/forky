use crate::resizable_canvas;
use leptos::html::Canvas;
use leptos::*;

#[component]
pub fn BevyCanvas(
	#[prop(optional)] canvas_ref: Option<NodeRef<Canvas>>,
) -> impl IntoView {
	let canvas_ref = if let Some(canvas_ref) = canvas_ref {
		canvas_ref
	} else {
		create_node_ref()
	};
	resizable_canvas(canvas_ref);

	view! { <canvas id="bevy-canvas" class="bevy-canvas" data-bevy="primary_window" node_ref=canvas_ref></canvas> }
}
