use forky_web::prelude::*;
use sweet::*;

#[sweet_test(skip)]
pub async fn html_event_listener() -> Result<()> {
    console_log::log!("waiting..");
    HtmlEventWaiter::new("click").wait().await.anyhow()?;
    console_log::log!("ok..");
    Ok(())
}
