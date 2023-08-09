// use forky_core::*;
use sweet::*;

sweet! {
	test skip e2e "same origin" {
			let page = visit("http://localhost:7777?m=%21").await?;
				expect(page)
					.poll(|p|p.to_contain_text("🤘 sweet as! 🤘")).await?;
		}
	test e2e "docs origin" {
			let page = visit("http://localhost:3000").await?;
				expect(page)
					.poll(|p|p.to_contain_text("Very early stagess warning")).await?;
		}
}
