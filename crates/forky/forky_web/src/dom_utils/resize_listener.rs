use js_sys::Array;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;
use web_sys::HtmlCanvasElement;
use web_sys::ResizeObserver;
use web_sys::ResizeObserverEntry;
use web_sys::ResizeObserverSize;


/// Resize listener
/// For use with leptos, ensure it is moved to on_cleanup to avoid being dropped
pub struct ResizeListener {
	pub observer: ResizeObserver,
	pub cb: Closure<dyn FnMut(Array, ResizeObserver)>,
}

impl ResizeListener {
	pub fn new<F>(el: &Element, mut f: F) -> Self
	where
		F: FnMut(&ResizeObserverEntry) + 'static,
	{
		let cb = Closure::wrap(Box::new(
			move |entries: Array, _observer: ResizeObserver| {
				let entry = entries.get(0);
				let entry = entry.dyn_ref::<ResizeObserverEntry>().unwrap();
				f(entry);
			},
		) as Box<dyn FnMut(Array, ResizeObserver)>);
		let observer =
			ResizeObserver::new(&cb.as_ref().unchecked_ref()).unwrap();
		observer.observe(el);
		Self { cb, observer }
	}

	pub fn forget(self) { std::mem::forget(self); }
	/// utility function for parsing the entry, usually this is what you want
	/// https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/contentBoxSize
	pub fn parse_entry(entry: &ResizeObserverEntry) -> (u32, u32) {
		let first = entry.content_box_size().get(0);
		let first = first.unchecked_ref::<ResizeObserverSize>();
		let width = first.inline_size();
		let height = first.block_size();
		(width as u32, height as u32)
	}
	pub fn resize_canvas(canvas: HtmlCanvasElement) -> Self {
		let canvas2 = canvas.clone();
		Self::new(canvas.unchecked_ref(), move |entry| {
			let content_rect = entry.content_rect();
			canvas2.set_width(content_rect.width() as u32);
			canvas2.set_height(content_rect.height() as u32);
		})
	}
}

impl Drop for ResizeListener {
	fn drop(&mut self) { self.observer.disconnect(); }
}


#[cfg(feature = "leptos")]
#[allow(unused)]
pub use leptos_resize::*;
#[cfg(feature = "leptos")]
pub mod leptos_resize {
	use crate::ResizeListener;
	use leptos::html::Div;
	use leptos::*;
	use std::ops::Deref;
	use web_sys::ResizeObserverEntry;

	pub fn create_resize_listener(
		el: NodeRef<Div>,
	) -> ReadSignal<Option<ResizeObserverEntry>> {
		let signal = create_rw_signal(None);
		let resize_listener = create_effect(move |_| {
			if let Some(container) = el.get() {
				let el = container.deref();
				let el: &web_sys::Element = el.as_ref();
				let listener = ResizeListener::new(el, move |entry| {
					signal.set(Some(entry.clone()));
				});
				Some(listener)
			} else {
				None
			}
		});

		on_cleanup(move || drop(resize_listener));

		signal.read_only()
	}
}




// pub fn sync_canvas_size(
// 	canvas: HtmlCanvasElement,
// ) -> FnClosure2<Array, ResizeObserver> {
// 	let canvas2 = canvas.clone();
// 	let cb = Closure::wrap(Box::new(
// 		move |entries: Array, _observer: ResizeObserver| {
// 			let entry = entries.get(0);
// 			let entry =
// 				entry.dyn_ref::<ResizeObserverEntry>().unwrap();
// 			// entry.
// 			let content_rect = entry.content_rect();
// 			canvas2.set_width(content_rect.width() as u32 - 1);
// 			canvas2.set_height(content_rect.height() as u32 - 1);
// 		},
// 	) as Box<dyn FnMut(Array, ResizeObserver)>);
// 	let observer = ResizeObserver::new(&cb.as_ref().unchecked_ref()).unwrap();
// 	observer.observe(canvas.unchecked_ref());
// 	cb
// }
