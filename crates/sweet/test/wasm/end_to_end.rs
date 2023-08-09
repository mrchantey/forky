// use forky_core::*;
use sweet::*;

sweet! {
	test e2e "identical origin" {
			let page = visit("?m=%21").await?;
				expect(page)
					.poll(|p|p.to_contain_text("🤘 sweet as! 🤘")).await?;
		}
	test only e2e "docs origin" {
			let page = visit("http://localhost:3000").await?;
				expect(page)
					.poll(|p|p.to_contain_text("🤘 sweet as! 🤘")).await?;
		}
}
