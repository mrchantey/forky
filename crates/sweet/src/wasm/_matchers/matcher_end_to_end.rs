use anyhow::Result;
use forky_web::*;
use web_sys::Document;
use web_sys::HtmlIFrameElement;

const NO_IFRAME: &str = r#"
iframe only available in e2e tests:

it e2e "works" {
  let page = visit(my_url)?.await;
}
"#;


pub async fn visit(url: &str) -> Result<HtmlIFrameElement> {
	match Document::x_query_selector::<HtmlIFrameElement>("iframe") {
		None => Err(anyhow::anyhow!(NO_IFRAME)),
		Some(iframe) => {
			let set_url = format!("/_proxy_set_/{url}");
			fetch(&set_url).await?;
			let url = format!("/_proxy_/{url}");
			iframe.x_set_source_async(&url).await;
			Ok(iframe)
		}
	}
}
