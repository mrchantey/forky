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
	test "expect_get" {
		create_view();
		expect_el("h1")?.to_contain_text("This is a heading")?;
	}

	test "async"{

		let _handle = set_timeout(||{
			mount(|cx|view!{cx,<div>"hello world!"</div>});
		},Duration::from_millis(100));

		poll_ok(||expect_el("div")).await?
			.to_contain_text("hello world!")?;

	}
}
