use std::fmt::{Display, Formatter};

use bevy::prelude::*;

// #[derive(Component)]
// pub struct NodeTag;


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


impl Display for SplineNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "SplineNode({})", self.0)
	}
}
