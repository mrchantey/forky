use crate::*;
use anyhow::Result;
use forky_web::*;
use web_sys::HtmlElement;

// impl<T> SweetInto<HtmlElement> for Option<T>
// where
// 	T: SweetInto<HtmlElement>,
// {
// 	fn sweet_into(&self) -> HtmlElement {
// 		self.as_ref().unwrap().sweet_into()
// 	}
// }

//ie for window()
impl<F, T> SweetInto<HtmlElement> for F
where
	F: Fn() -> Option<T>,
	T: SweetInto<HtmlElement>,
{
	fn sweet_into(&self) -> HtmlElement { self().unwrap().sweet_into() }
}
// impl<T> SweetInto<HtmlElement> for fn() -> Option<T>
// where
// 	T: SweetInto<HtmlElement>,
// {
// 	fn sweet_into(&self) -> HtmlElement {
// 		self().unwrap().sweet_into()
// 	}
// }
// impl<T> SweetInto<HtmlElement> for T
// where
// 	T: SweetInto<HtmlElement>,
// {
// 	fn sweet_into(&self) -> HtmlElement { (*self).sweet_into() }
// }

impl SweetInto<HtmlElement> for web_sys::HtmlElement {
	fn sweet_into(&self) -> HtmlElement { self.clone() }
}
impl SweetInto<HtmlElement> for web_sys::Document {
	fn sweet_into(&self) -> HtmlElement { self.body().unwrap() }
}

impl SweetInto<HtmlElement> for web_sys::Window {
	fn sweet_into(&self) -> HtmlElement {
		self.document().unwrap().sweet_into()
	}
}
impl SweetInto<HtmlElement> for web_sys::HtmlIFrameElement {
	fn sweet_into(&self) -> HtmlElement {
		self.content_document().unwrap().sweet_into()
	}
}

pub trait MatcherHtml<T>: MatcherTrait<T>
where
	T: SweetInto<HtmlElement>,
{
	fn get(&self, selector: &str) -> Result<Matcher<HtmlElement>> {
		let matcher = self.get_matcher();
		let parent = matcher.value.sweet_into();
		// let expected = format!(
		// 	"element {} to contain selector '{selector}'",
		// 	parent.tag_name()
		// );
		let received = parent.x_query_selector::<HtmlElement>(selector);
		matcher
			.assert_option_with_received(received)
			.map(|c| Matcher::new(c))
	}

	fn to_contain_text(&self, other: &str) -> Result<()> {
		let receive = self
			.get_value()
			.sweet_into()
			.text_content()
			.unwrap_or_default();
		self.contains(other, &receive, "text")
	}
	fn to_contain_visible_text(&self, other: &str) -> Result<()> {
		let receive = self.get_value().sweet_into().inner_text();
		self.contains(other, &receive, "visible text")
	}
	fn to_contain_html(&self, other: &str) -> Result<()> {
		let receive = self.get_value().sweet_into().inner_html();
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
			received.push_str("...TRUNCATED...");
		}
		let expected = format!("to contain {} '{}'", expect_suffix, other);

		self.get_matcher()
			.assert_correct_with_received(result, &expected, &received)
	}
}

impl<T> MatcherHtml<T> for Matcher<T> where T: SweetInto<HtmlElement> {}
