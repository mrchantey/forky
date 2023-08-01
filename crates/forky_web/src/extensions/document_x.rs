use extend::ext;
use wasm_bindgen::JsCast;
use web_sys::*;

#[ext]
pub impl Document {
	fn get() -> Document { window().unwrap().document().unwrap() }
	fn x_body() -> HtmlElement { Self::get().body().unwrap() }

	fn x_append_child(node: &Node) {
		Self::x_body().append_child(node).unwrap();
	}

	fn x_clear() {
		let body = Self::x_body();
		while let Some(child) = body.first_child() {
			body.remove_child(&child).unwrap();
		}
	}

	fn x_query_selector<T>(selector: &str) -> Option<T>
	where
		T: JsCast,
	{
		Self::get()
			.query_selector(selector)
			.unwrap()
			.map(|el| el.dyn_into::<T>().unwrap())
	}
	fn x_create_element(local_name: &str) -> HtmlElement {
		Self::get()
			.create_element(local_name)
			.unwrap()
			.dyn_into::<HtmlElement>()
			.unwrap()
	}
	fn x_create_anchor() -> HtmlAnchorElement {
		Self::get()
			.create_element("a")
			.unwrap()
			.dyn_into::<HtmlAnchorElement>()
			.unwrap()
	}
	fn x_create_canvas() -> HtmlCanvasElement {
		Self::get()
			.create_element("canvas")
			.unwrap()
			.dyn_into::<HtmlCanvasElement>()
			.unwrap()
	}
	fn x_create_div() -> HtmlDivElement {
		Self::get()
			.create_element("div")
			.unwrap()
			.dyn_into::<HtmlDivElement>()
			.unwrap()
	}
	fn x_create_input() -> HtmlInputElement {
		Self::get()
			.create_element("input")
			.unwrap()
			.dyn_into::<HtmlInputElement>()
			.unwrap()
	}
	fn x_create_paragraph() -> HtmlParagraphElement {
		Self::get()
			.create_element("p")
			.unwrap()
			.dyn_into::<HtmlParagraphElement>()
			.unwrap()
	}
}
