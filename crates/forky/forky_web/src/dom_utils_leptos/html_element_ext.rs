use leptos::html::ElementDescriptor;
use leptos::*;


#[extend::ext]
pub impl<T: 'static + ElementDescriptor> leptos::HtmlElement<T> {
	fn into_web_sys(self) -> web_sys::HtmlElement {
		let el = self.into_any();
		let el: &web_sys::HtmlElement = el.as_ref();
		el.clone()
		// el.into()
	}
}

