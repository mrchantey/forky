use crate::*;
use anyhow::Result;
use forky_web::*;
use web_sys::HtmlElement;

impl<F, T> SweetBorrow<HtmlElement> for F
where
	F: Fn() -> Option<T>,
	T: SweetBorrow<HtmlElement>,
{
	fn sweet_borrow(&self) -> HtmlElement { self().unwrap().sweet_borrow() }
}

impl SweetBorrow<HtmlElement> for web_sys::HtmlElement {
	fn sweet_borrow(&self) -> HtmlElement { self.clone() }
}
impl SweetBorrow<HtmlElement> for web_sys::Document {
	fn sweet_borrow(&self) -> HtmlElement { self.body().unwrap() }
}

impl SweetBorrow<HtmlElement> for web_sys::Window {
	fn sweet_borrow(&self) -> HtmlElement {
		self.document().unwrap().sweet_borrow()
	}
}
impl SweetBorrow<HtmlElement> for web_sys::HtmlIFrameElement {
	fn sweet_borrow(&self) -> HtmlElement {
		self.content_document().unwrap().sweet_borrow()
	}
}

pub trait MatcherHtml<T>: MatcherTrait<T>
where	T: SweetBorrow<HtmlElement>,
{
	fn get(&self, selector: &str) -> Result<Matcher<HtmlElement>> {
		let matcher = self.get_matcher();
		let parent = matcher.value.sweet_borrow();
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
			.sweet_borrow()
			.text_content()
			.unwrap_or_default();
		self.contains(other, &receive, "text")
	}
	fn to_contain_visible_text(&self, other: &str) -> Result<()> {
		let receive = self.get_value().sweet_borrow().inner_text();
		self.contains(other, &receive, "visible text")
	}
	fn to_contain_html(&self, other: &str) -> Result<()> {
		let receive = self.get_value().sweet_borrow().inner_html();
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

impl<T> MatcherHtml<T> for Matcher<T> where T: SweetBorrow<HtmlElement> {}

// impl<T> SweetBorrow<HtmlElement> for fn() -> Option<T>
// where
// 	T: SweetBorrow<HtmlElement>,
// {
// 	fn sweet_borrow(&self) -> HtmlElement {
// 		self().unwrap().sweet_borrow()
// 	}
// }
// impl<T> SweetBorrow<HtmlElement> for T
// where
// 	T: SweetBorrow<HtmlElement>,
// {
// 	fn sweet_borrow(&self) -> HtmlElement { (*self).sweet_borrow() }
// }

// impl<T> SweetBorrow<HtmlElement> for Option<T>
// where
// 	T: SweetBorrow<HtmlElement>,
// {
// 	fn sweet_borrow(&self) -> HtmlElement {
// 		self.as_ref().unwrap().sweet_borrow()
// 	}
// }

//ie for window()
