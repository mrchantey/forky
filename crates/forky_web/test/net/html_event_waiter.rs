use forky_web::prelude::*;
use sweet::*;

#[sweet_test(skip)]
pub async fn html_event_listener() -> Result<()> {
	leptos::logging::log!("thats ok");
	HtmlEventWaiter::new("click").wait().await.anyhow()?;
	leptos::logging::log!("thats ok");

	Ok(())
}
