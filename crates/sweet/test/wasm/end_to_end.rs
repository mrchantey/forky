// use forky_core::*;
use sweet::*;

sweet! {
	test e2e "same origin" {
			let page = visit("?m=%21").await?;
				expect(page)
					.poll(|p|p.to_contain_text("🤘 sweet as! 🤘")).await?;
		}
	test only e2e "docs origin" {
			let page = visit("http://localhost:7779").await?;
				expect(page)
					.poll(|p|p.to_contain_text("🤘 sweet as! 🤘")).await?;
		}
}
