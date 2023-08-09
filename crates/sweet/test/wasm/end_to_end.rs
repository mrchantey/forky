// use forky_core::*;
use sweet::*;

sweet! {
	it e2e "works" {
		let page = visit("?m=%21").await?;
		(move ||{
			let inner_text = page.content_window().unwrap().document().unwrap().body().unwrap().inner_text();
			expect(inner_text.as_str()).to_contain("🤘 sweet as! 🤘")
		}).poll().await?;
	}
}
