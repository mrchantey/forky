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
