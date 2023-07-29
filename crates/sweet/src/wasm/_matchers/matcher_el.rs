use crate::*;
use anyhow::Result;
use web_sys::HtmlElement;


impl Matcher<HtmlElement> {
	pub fn to_contain_text(&self, other: &str) -> Result<()> {
		let receive = self.value.text_content().unwrap_or_default();
		self.contains(other, &receive, "text")
	}
	pub fn to_contain_visible_text(&self, other: &str) -> Result<()> {
		let receive = self.value.inner_text();
		self.contains(other, &receive, "visible text")
	}
	pub fn to_contain_html(&self, other: &str) -> Result<()> {
		let receive = self.value.inner_html();
		self.contains(other, &receive, "html")
	}
	fn contains(
		&self,
		other: &str,
		receive: &str,
		expect_suffix: &str,
	) -> Result<()> {
		let result = receive.contains(other);
		let received = receive.chars().take(100).collect::<String>();
		let expected = format!("contains {}: {}", expect_suffix, other);
		self.assert_correct_with_received(result, &expected, &received)
	}
}
