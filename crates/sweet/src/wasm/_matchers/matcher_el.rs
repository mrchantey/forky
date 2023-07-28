use web_sys::Element;


impl Matcher<Element> {
	pub fn to_contain_text(&self, other: &str) -> Result<()> {
		if self.value.contains(other) {
			Ok(())
		} else {
			let expect = format!("contains text {}", other);
			let receive = "no good";
			Err(MatcherError::new(expect, receive, 0))
		}
	}
}

fn search_element_for_string(element: &Element, search_string: &str) -> bool {
	let text_content = element.text_content().unwrap_or_default();
	let re = RegExp::new(search_string).unwrap();
	if re.test(&text_content) {
		return true;
	}

	let children = element.children();
	for i in 0..children.length() {
		if let Some(child) = children.item(i) {
			if search_element_for_string(
				&child.dyn_ref::<Element>().unwrap(),
				search_string,
			) {
				return true;
			}
		}
	}

	false
}
