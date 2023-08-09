use web_sys::*;

pub struct Page {
	pub iframe: HtmlIFrameElement,
}

impl Page {
	pub fn document(&self) -> Document {
		self.iframe.content_document().unwrap()
	}
	pub fn body(&self) -> HtmlElement {
		self.iframe.content_document().unwrap().body().unwrap()
	}
	pub fn window(&self) -> Window { self.iframe.content_window().unwrap() }
}
