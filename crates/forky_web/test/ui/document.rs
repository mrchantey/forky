use forky_web::*;
use sweet::*;
use web_sys::*;

sweet! {
	it "works" {

		let div = Document::x_create_div();
		div.set_inner_html("hello world");
		Document::x_append_child(&div);

		expect(window.into_html_element()).to_contain_text("hello world")?;
	}
}
