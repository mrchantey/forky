// use bevy::prelude::*;
// use bevy::prelude::Component;
use std::fmt::Debug;
// use std::marker::PhantomData;
// use std::ops::Deref;
// use std::ops::DerefMut;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
	#[default]
	Once,
	Repeating,
	//yield? as in happy to continue but check choices again
}

impl NodeState {
	pub fn finish(&mut self) { *self = NodeState::Once; }
}
