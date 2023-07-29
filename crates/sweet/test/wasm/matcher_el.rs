use forky_core::wasm::*;
use leptos::*;
use sweet::*;
use std::time::Duration;

sweet! {
	it "works" {

		let _m = mount_with_unmount(|cx|view!{cx,
			<div>
			<h1>"This is a heading"</h1>
			<p>"This is a paragraph"</p>
			<p style="display:none">"This is hidden"</p>
			</div>
		});
		expect_body().to_contain_text("This is a headingThis is a paragraph")?;
		expect_body().not().to_contain_text("foobar")?;
		expect_body().to_contain_text("This is hidden")?;
		expect_body().not().to_contain_visible_text("This is hidden")?;
		expect_body().to_contain_html("<div><h1>This is a heading</h1>")?;
	}

	test "async"{
		
		let _handle = set_timeout(||{
			mount(|cx|view!{cx,<div>"hello world!"</div>});
		},Duration::from_millis(100));

		poll_ok(||expect_body().to_contain_text("hello world!")).await?;

	}
}
