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
	pub handle: SplineNodeHandle,
}

impl SplineNodeBundle {
	pub fn new(position: Vec3, node: SplineNode) -> Self {
		SplineNodeBundle {
			transform: TransformBundle::from(Transform::from_translation(
				position,
			)),
			node,
			handle:SplineNodeHandle::default()
		}
	}
}

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct SplineNodeHandle {
	pub edges: Vec<Entity>,
}


pub fn on_node_moved(
	query: Query<(&Transform, &SplineNode), Changed<Transform>>,
) {
	for (transform, node) in query.iter() {
		println!("Node {} moved to {:?}", node.0, transform.translation);
	}
}
