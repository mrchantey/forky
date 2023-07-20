use js_sys::Array;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;
use web_sys::HtmlCanvasElement;
use web_sys::ResizeObserver;
use web_sys::ResizeObserverEntry;

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
