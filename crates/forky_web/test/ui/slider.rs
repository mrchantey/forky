use forky_web::*;
use leptos::*;
use sweet::*;

#[component]
fn TestSlider(cx: Scope) -> impl IntoView {
	let (value, set_value) = create_signal(cx, 50.);

	view! {cx,
		<div class=forky_style::FLEX_COL style="width:30em">
			<div>
			<div>"value: "{value()}</div>
			<Slider value=value set_value=set_value/>
			<TextSlider value=value set_value=set_value/>
			</div>
		</div>
	}
}

sweet! {
	it "works" {
		mount(|cx| view! {cx,
			<>
			<TestSlider/>
		</>
		});
		expect(web_sys::window).to_contain_text("value: 50")?;

	}

	it skip "fails"{

		expect(true).to_be_false()?;
	}
}
