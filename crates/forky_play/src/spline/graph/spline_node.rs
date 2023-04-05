use bevy::prelude::*;
use std::cmp::{Ordering, PartialOrd};
use std::collections::hash_map::DefaultHasher;
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
pub struct SplineNode {
	pub id: u64,
	pub position: Vec3,
}
impl Eq for SplineNode {}

impl PartialEq for SplineNode {
	fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl Ord for SplineNode {
	fn cmp(&self, other: &Self) -> Ordering { self.id.cmp(&other.id) }
}

impl PartialOrd for SplineNode {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Hash for SplineNode {
	fn hash<H: Hasher>(&self, state: &mut H) { self.id.hash(state); }
}
