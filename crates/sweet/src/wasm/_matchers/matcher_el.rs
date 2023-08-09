use crate::*;
use anyhow::Result;
use forky_web::*;
use web_sys::HtmlElement;


pub trait IntoHtmlElement {
	fn into_html_element(&self) -> HtmlElement;
}

impl<T> IntoHtmlElement for Option<T>
where
	T: IntoHtmlElement,
{
	fn into_html_element(&self) -> HtmlElement {
		self.as_ref().unwrap().into_html_element()
	}
}
impl<F, T> IntoHtmlElement for F
where
	F: Fn() -> Option<T>,
	T: IntoHtmlElement,
{
	fn into_html_element(&self) -> HtmlElement {
		self().unwrap().into_html_element()
	}
}
// impl<T> IntoHtmlElement for fn() -> Option<T>
// where
// 	T: IntoHtmlElement,
// {
// 	fn into_html_element(&self) -> HtmlElement {
// 		self().unwrap().into_html_element()
// 	}
// }
// impl<T> IntoHtmlElement for T
// where
// 	T: IntoHtmlElement,
// {
// 	fn into_html_element(&self) -> HtmlElement { (*self).into_html_element() }
// }

impl IntoHtmlElement for web_sys::HtmlElement {
	fn into_html_element(&self) -> HtmlElement { self.clone() }
}
impl IntoHtmlElement for web_sys::Document {
	fn into_html_element(&self) -> HtmlElement { self.body().unwrap() }
}

impl IntoHtmlElement for web_sys::Window {
	fn into_html_element(&self) -> HtmlElement {
		self.document().unwrap().into_html_element()
	}
}
impl IntoHtmlElement for web_sys::HtmlIFrameElement {
	fn into_html_element(&self) -> HtmlElement {
		self.content_document().unwrap().into_html_element()
	}
}

pub trait MatcherHtml<T>: MatcherTrait<T>
where
	T: IntoHtmlElement,
{
	fn get(&self, selector: &str) -> Result<Matcher<Option<HtmlElement>>> {
		let matcher = self.get_matcher();
		let parent = matcher.value.into_html_element();
		let expected = format!(
			"element {} to contain selector '{selector}'",
			parent.tag_name()
		);
		let received = parent.x_query_selector::<HtmlElement>(selector);
		match matcher.assert_option_with_received(&expected, received) {
			Ok(value) => Ok(Matcher::new(value)),
			Err(e) => Err(e),
		}
	}

	fn to_contain_text(&self, other: &str) -> Result<()> {
		let receive = self
			.get_value()
			.into_html_element()
			.text_content()
			.unwrap_or_default();
		self.contains(other, &receive, "text")
	}
	fn to_contain_visible_text(&self, other: &str) -> Result<()> {
		let receive = self.get_value().into_html_element().inner_text();
		self.contains(other, &receive, "visible text")
	}
	fn to_contain_html(&self, other: &str) -> Result<()> {
		let receive = self.get_value().into_html_element().inner_html();
		self.contains(other, &receive, "html")
	}
	fn contains(
		&self,
		other: &str,
		receive: &str,
		expect_suffix: &str,
	) -> Result<()> {
		let result = receive.contains(other);
		// forky_core::log!("result: {result}");
		let mut received = receive.chars().take(100).collect::<String>();
		if received.len() == 100 {
			received.push_str("...truncated...");
		}
		let expected = format!("to contain {} '{}'", expect_suffix, other);

		self.get_matcher()
			.assert_correct_with_received(result, &expected, &received)
	}
}

impl<T> MatcherHtml<T> for Matcher<T> where T: IntoHtmlElement {}
