use forky_core::wasm::*;
use leptos::*;
use sweet::*;

sweet! {
	it "works" {
		
		mount(|cx|view!{cx,
			<div>
			<h1>"This is a heading"</h1>
			<p>"This is a paragraph."</p>
			</div>
		});
		// expect(true).to_be_false()?;

		expect_body().not().to_contain_text("This is a heading")?;
	}
}
