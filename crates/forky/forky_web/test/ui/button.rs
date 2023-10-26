use forky_web::*;
use leptos::*;
use sweet::*;

sweet! {
	it "works" {
		mount(|cx| view! {cx,
			<Button>"click me!"</Button>
		});
	}
}
