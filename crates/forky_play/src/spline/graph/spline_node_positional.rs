use bevy::prelude::*;
use std::cmp::{Ordering, PartialOrd};
use std::hash::{Hash, Hasher};
// #[derive(Component)]
// pub struct NodeTag;


#[derive(
	Component,
	// Deref,
	// DerefMut,
	Debug,
	Copy,
	Clone,
	// Eq,
	// PartialEq,
	// Ord,
	// PartialOrd,
	// Hash,
)]
pub struct SplineNodePositional {
	pub id: u64,
	pub position: Vec3,
}

impl SplineNodePositional {
	pub fn new(id: u64, position: Vec3) -> Self { Self { id, position } }
}

impl Eq for SplineNodePositional {}

impl PartialEq for SplineNodePositional {
	fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl Ord for SplineNodePositional {
	fn cmp(&self, other: &Self) -> Ordering { self.id.cmp(&other.id) }
}

impl PartialOrd for SplineNodePositional {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Hash for SplineNodePositional {
	fn hash<H: Hasher>(&self, state: &mut H) { self.id.hash(state); }
}
