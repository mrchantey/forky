// use forky_core::*;
use sweet::*;

sweet! {
it e2e "works" {
		let page = visit("?m=%21").await?;
		(move ||{
			expect(page.clone()).to_contain_text("🤘 sweet as! 🤘")
		}).poll().await?;
	}
}
