use crate::*;
use bevy_ecs::prelude::*;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Default, SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
/// Runs before parent update
pub struct PreParentUpdateSet<Path: TreePath> {
	phantom: PhantomData<Path>,
}
impl<Path: TreePath> PreParentUpdateSet<Path> {
	pub fn new() -> Self {
		Self {
			phantom: PhantomData,
		}
	}
}

#[derive(Default, SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
/// Runs after parent update, before update
pub struct PreUpdateSet<Path: TreePath> {
	phantom: PhantomData<Path>,
}

impl<Path: TreePath> PreUpdateSet<Path> {
	pub fn new() -> Self {
		Self {
			phantom: PhantomData,
		}
	}
}

#[derive(Default, SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
/// Main node update set
pub struct UpdateSet<Path: TreePath> {
	phantom: PhantomData<Path>,
}
impl<Path: TreePath> UpdateSet<Path> {
	pub fn new() -> Self {
		Self {
			phantom: PhantomData,
		}
	}
}

#[derive(Default, SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
/// Runs after update, before child update & same time as grandchild preparentupdate
pub struct PostUpdateSet<Path: TreePath> {
	phantom: PhantomData<Path>,
}
impl<Path: TreePath> PostUpdateSet<Path> {
	pub fn new() -> Self {
		Self {
			phantom: PhantomData,
		}
	}
}

pub trait IntoNodeSets: TreePath + Clone + Debug {
	// type Path: TreePath + Clone + Debug;

	fn pre_parent_update_set() -> impl SystemSet {
		PreParentUpdateSet::<Self>::new()
	}
	fn pre_update_set() -> impl SystemSet { PreUpdateSet::<Self>::new() }
	fn update_set() -> impl SystemSet { UpdateSet::<Self>::new() }
	fn post_update_set() -> impl SystemSet { PostUpdateSet::<Self>::new() }
	fn configure_sets(schedule: &mut Schedule) {
		schedule.configure_set(
			PreParentUpdateSet::<Self>::new()
				.before(UpdateSet::<Self::Parent>::new()),
		);
		schedule.configure_set(
			PreUpdateSet::<Self>::new()
				.before(UpdateSet::<Self>::new())
				.after(UpdateSet::<Self::Parent>::new()),
		);
		schedule.configure_set(
			PostUpdateSet::<Self>::new().after(UpdateSet::<Self>::new()),
		);
	}
}

// all tree paths implement intonodesets
impl<Path: TreePath> IntoNodeSets for Path {}
