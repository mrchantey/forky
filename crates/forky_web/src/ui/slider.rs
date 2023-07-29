use super::*;
use forky_core::*;
use leptos::*;

#[component]
pub fn Slider(
	cx: Scope,
	#[prop(into)] value: ReadSignal<f32>,
	#[prop(into)] set_value: WriteSignal<f32>,
) -> impl IntoView {
	view! {cx,
		<input
		type="range"
		class={spacecat!(ui_style::SLIDER_RANGE_INPUT,ui_style::SLIDER_CONTAINER)}
		on:input=move |ev| {
			if let Ok(value) = event_target_value(&ev).parse::<f32>() {
				set_value(value);
			}
		}
		prop:value=value
		/>
	}
}

#[component]
pub fn TextSlider(
	cx: Scope,
	#[prop(into)] value: ReadSignal<f32>,
	#[prop(into)] set_value: WriteSignal<f32>,
) -> impl IntoView {
	// let (value, set_value) = create_signal(cx, 50.);

	view! {cx,
		<div class=ui_style::SLIDER_CONTAINER>
		<input
			type="text"
			class=ui_style::SLIDER_TEXT_INPUT
			on:input=move |ev| {
				if let Ok(value) = event_target_value(&ev).parse::<f32>() {
					set_value(value);
				}
			}
			prop:value=value
			/>
		<input
			type="range"
			class=ui_style::SLIDER_RANGE_INPUT
			on:input=move |ev| {
				if let Ok(value) = event_target_value(&ev).parse::<f32>() {
					set_value(value);
				}
			}
			prop:value=value
			/>
		</div>
	}
}
