use forky_web::*;
use leptos::*;
use std::time::Duration;
use sweet::*;
use web_sys::window;

fn create_view() {
	mount(|cx| {
		view! {cx,
			<div>
			<h1>"This is a heading"</h1>
			<p>"This is a paragraph"</p>
			<p style="display:none">"This is hidden"</p>
			</div>
		}
	})
}

sweet! {
	test "expect window" {
		create_view();
		expect(window).to_contain_text("This is a headingThis is a paragraph")?;
		expect(window).not().to_contain_text("foobar")?;
		expect(window).to_contain_text("This is hidden")?;
		expect(window).not().to_contain_visible_text("This is hidden")?;
		expect(window).to_contain_html("<div><h1>This is a heading</h1>")?;
	}
	test "get" {
		expect(window).not().get("div")?;
		create_view();
		expect(window).get("div")?;
		expect(window).get("h1")?
			.to_contain_text("This is a heading")?;
	}

	test "async"{

		let _handle = set_timeout(||{
			mount(|cx|view!{cx,<div>"hello world!"</div>});
		},Duration::from_millis(10));

		expect(window).not().get("div")?;

		expect(window).poll(|w|w.get("div")).await?
			// .not().to_contain_text("hello world!")?;
			.to_contain_text("hello world!")?;
	}
}
