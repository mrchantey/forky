use bevy::prelude::*;

/// Bundle to complement the `TransformBundle`
#[derive(Bundle, Clone, Default)]
pub struct RenderBundle<M: Material> {
	pub mesh: Handle<Mesh>,
	pub material: Handle<M>,
	pub visibility: Visibility,
	pub computed_visibility: ComputedVisibility,
}
