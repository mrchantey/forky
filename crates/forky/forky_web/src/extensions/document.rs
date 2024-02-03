use crate::prelude::*;
use extend::ext;
use forky_core::OptionTExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;

const ERROR: &str = "Very basic dom query error, this is very weird.";

#[ext]
pub impl Document {
	fn get() -> Document { window().expect(ERROR).document().expect(ERROR) }
	fn x_head() -> HtmlHeadElement { Self::get().head().expect(ERROR) }
	fn x_body() -> HtmlElement { Self::get().body().expect(ERROR) }

	async fn x_await_load_by_id(id: &str) -> anyhow::Result<()> {
		HtmlEventListener::wait_with_target(
			"load",
			Self::get().get_element_by_id(id).ok()?.dyn_into().unwrap(),
		)
		.await;
		Ok(())
	}

	fn x_append_child(node: &Node) {
		Self::x_body().append_child(node).expect(ERROR);
	}

	fn x_clear() {
		let body = Self::x_body();
		while let Some(child) = body.first_child() {
			body.remove_child(&child).expect(ERROR);
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


	fn add_script_src_to_head(src: &str) -> Result<HtmlScriptElement, JsValue> {
		let el = Document::get()
			.create_element("script")
			.unwrap()
			.dyn_into::<HtmlScriptElement>()?;
		el.set_src(src);
		el.set_type("text/javascript");
		Document::x_head().append_child(&el).unwrap();
		Ok(el)
	}

	fn add_script_content_to_body(
		body: &str,
	) -> Result<HtmlScriptElement, JsValue> {
		let el = Document::get()
			.create_element("script")
			.unwrap()
			.dyn_into::<HtmlScriptElement>()?;
		el.set_type("text/javascript");
		el.set_inner_html(body);
		Document::x_body().append_child(&el).unwrap();
		Ok(el)
	}

	fn add_style_src_to_head(src: &str) -> Result<HtmlLinkElement, JsValue> {
		let el = Document::get()
			.create_element("link")
			.unwrap()
			.dyn_into::<HtmlLinkElement>()?;
		el.set_href(src);
		el.set_rel("stylesheet");
		el.set_type("text/css");
		Document::x_head().append_child(&el).unwrap();
		Ok(el)
	}
}
