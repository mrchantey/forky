use bevy::prelude::*;
use derive_deref::{Deref, DerefMut};

// #[derive(Component, Default)]
// pub struct HoverActive {}
// #[derive(Component, Default)]
// pub struct HoverInactive {}
#[derive(Component, Default)]
pub struct Selected {}
// #[derive(Component, Default)]
// pub struct SelectInactive {}

#[derive(Component, Deref, DerefMut)]
pub struct InteractColor(pub Color);

impl Default for InteractColor {
	fn default() -> Self { Self(INACTIVE_COLOR) }
}

pub const INACTIVE_COLOR: Color = Color::rgb(0., 1., 0.);
pub const HOVER_COLOR: Color = Color::rgb(0., 1., 1.);
pub const SELECT_COLOR: Color = Color::rgb(0., 0., 1.);

pub const SELECT_BUTTON: MouseButton = MouseButton::Left;

pub fn on_interact_color_change(
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut query: Query<
		(&Handle<StandardMaterial>, &InteractColor),
		Changed<InteractColor>,
	>,
) {
	for (handle, color) in query.iter_mut() {
		let mut material = materials.get_mut(&handle).unwrap();
		material.base_color = **color;
	}
}
