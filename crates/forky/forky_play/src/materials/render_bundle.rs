use bevy::prelude::*;

/// Bundle to complement the `TransformBundle`
#[derive(Bundle, Default)]
pub struct RenderBundle<M: Material> {
	pub mesh: Handle<Mesh>,
	pub material: Handle<M>,
	pub visibility: VisibilityBundle,
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
		}
	}
}
