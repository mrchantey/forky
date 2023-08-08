use forky_web::*;
use web_sys::Document;
use web_sys::HtmlIFrameElement;
use anyhow::Result;

const NO_IFRAME:&str = r#"
iframe only available in e2e tests:

it e2e "works" {
  let page = visit(my_url)?.await;
}
"#;


pub async fn visit(url: &str)->Result<()> {
	match Document::x_query_selector::<HtmlIFrameElement>("iframe"){
		None=>Err(anyhow::anyhow!(NO_IFRAME)),
		Some(iframe)=>{
			iframe.set_src(url);
			//TODO await
			Ok(())
		}
	}
}
