use super::*;
use forky_core::*;
use leptos::*;

#[component]
pub fn Slider(
	#[prop(into)] value: ReadSignal<f32>,
	#[prop(into)] set_value: WriteSignal<f32>,
) -> impl IntoView {
	view! {
		<input
			type="range"
			class=spacecat!(ui_style::SLIDER_RANGE_INPUT, ui_style::SLIDER_CONTAINER)
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
	#[prop(default = 0.)] min: f32,
	#[prop(default = 1.)] max: f32,
	#[prop(default = 0.01)] step: f32,
	#[prop(into)] value: ReadSignal<f32>,
	#[prop(into)] set_value: WriteSignal<f32>,
) -> impl IntoView {
	// let (value, set_value) = create_signal(50.);

	let text_value = move || {
		let value = value();
		let rounded_value = (value / step).round() * step;
		if rounded_value.fract() == 0. {
			format!("{}", rounded_value as i32)
		} else {
			format!("{:.2}", rounded_value)
		}
	};
	view! {
		<div class=ui_style::SLIDER_CONTAINER>
			<input
				type="text"
				class=ui_style::SLIDER_TEXT_INPUT
				on:input=move |ev| {
					if let Ok(value) = event_target_value(&ev).parse::<f32>() {
						set_value(value);
					}
				}

				prop:value=text_value
			/>
			<input
				type="range"
				min=min
				max=max
				step=step
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
