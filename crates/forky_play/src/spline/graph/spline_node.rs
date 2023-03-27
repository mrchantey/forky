use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use derive_deref::{Deref, DerefMut};

#[derive(
	Component,
	Deref,
	DerefMut,
	Debug,
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Hash,
)]
pub struct SplineNode(pub u64);

#[derive(Bundle)]
pub struct SplineNodeBundle {
	pub node: SplineNode,
	pub transform: TransformBundle,
}

impl SplineNodeBundle {
	pub fn new(position: Vec3, node: SplineNode) -> Self {
		SplineNodeBundle {
			transform: TransformBundle::from(Transform::from_translation(
				position,
			)),
			node,
		}
	}
}
