use bevy::prelude::*;

/// Bundle to complement the `TransformBundle`
#[derive(Bundle, Clone, Default)]
pub struct RenderBundle<M: Material> {
	pub mesh: Handle<Mesh>,
	pub material: Handle<M>,
	pub visibility: Visibility,
	pub computed_visibility: ComputedVisibility,
}


impl<M> RenderBundle<M>
where
	M: Material,
{
	pub fn new(mesh: Handle<Mesh>, material: Handle<M>) -> Self {
		Self {
			mesh,
			material,
			visibility: default(),
			computed_visibility: default(),
		}
	}
}
