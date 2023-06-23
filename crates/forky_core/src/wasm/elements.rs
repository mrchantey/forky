use js_sys::Array;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Element, HtmlCanvasElement, HtmlElement, ResizeObserver};

pub fn create_element(name: &str) -> Result<Element, JsValue> {
	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();
	let el = document.create_element(name)?;
	// let el = el.dyn_into::<HtmlElement>()?;
	document.body().unwrap().append_child(&el)?;
	Ok(el)
}

pub fn create_div() -> Result<HtmlElement, JsValue> {
	let el = create_element("div")?.dyn_into::<HtmlElement>()?;
	Ok(el)
}

pub fn create_canvas() -> Result<HtmlCanvasElement, JsValue> {
	let el = create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
	Ok(el)
}


pub fn sync_canvas_size(canvas: HtmlCanvasElement) {
	let canvas2 = canvas.clone();
	let cb = Closure::wrap(Box::new(
		move |entries: Array, _observer: ResizeObserver| {
			let entry = entries.get(0);
			let content_rect = entry
				.dyn_ref::<web_sys::ResizeObserverEntry>()
				.unwrap()
				.content_rect();
			canvas2.set_width(content_rect.width() as u32);
			canvas2.set_height(content_rect.height() as u32);
		},
	) as Box<dyn FnMut(Array, ResizeObserver)>);
	let observer = ResizeObserver::new(&cb.as_ref().unchecked_ref()).unwrap();
	cb.forget();
	observer.observe(canvas.unchecked_ref());
}
