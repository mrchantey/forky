// use forky_core::*;
use sweet::*;

sweet! {
it e2e "works" {
		let page = visit("?m=%21").await?;
			expect(page)
				.poll(|p|p.to_contain_text("🤘 sweet as! 🤘")).await?;
		// }).poll().await?;
	}
}
