use forky_core::*;
use sweet::*;

sweet! {
	it e2e "works" {
		let page = visit("?m=%21").await?;
		// log!("{inner_html}");
		poll_ok(move ||{
			if let Some(body) = page.content_window().unwrap().document().unwrap().body(){
				let inner_text = body.inner_text();
				expect(inner_text.clone().as_str()).to_contain("🤘 sweet as! 🤘")
			}else{
			expect(true).to_be_false()
			}
		}).await?;
		// expect(true).to_be_false()?;
	}
}
