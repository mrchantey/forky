use forky_web::*;
use leptos::*;
use sweet::*;

sweet! {
	it "works" {
		mount(|| view! {
			<Button>"click me!"</Button>
		});
	}
}
