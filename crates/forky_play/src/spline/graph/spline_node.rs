use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use derive_deref::{Deref, DerefMut};

#[derive(
	Deref, DerefMut, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
pub struct SplineNode(pub u64);



#[derive(Bundle, Default)]
pub struct SplineNodeBundle {
	pub pbr: PbrBundle,
	pub collider: Collider,
	pub interact_color: spline::tool::InteractColor,
	//todo reference node
}

impl SplineNodeBundle {
	pub fn new(
		position: Vec3,
		mesh: &Handle<Mesh>,
		materials: &mut ResMut<Assets<StandardMaterial>>,
		radius: f32,
	) -> Self {
		SplineNodeBundle {
			pbr: PbrBundle {
				transform: Transform::from_translation(position),
				mesh: mesh.clone(),
				material: materials.add(Color::BLACK.into()),
				..default()
			},
			collider: Collider::ball(radius),
			..default()
		}
	}
}
