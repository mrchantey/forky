use crate::*;
use bevy_ecs::prelude::*;
use std::fmt::Debug;

pub trait IntoNodeSets: TreePath + Clone + Debug {
	// type Path: TreePath + Clone + Debug;

	fn pre_parent_update_set() -> impl SystemSet {
		ActionSet::pre_parent_update::<Self>()
	}
	fn pre_update_set() -> impl SystemSet { ActionSet::pre_update::<Self>() }
	fn update_set() -> impl SystemSet { ActionSet::update::<Self>() }
	fn post_update_set() -> impl SystemSet { ActionSet::post_update::<Self>() }
	fn configure_sets(schedule: &mut Schedule) {
		if Self::DEPTH > 1 {
			if Self::DEPTH > 2 {
				schedule.configure_set(
					Self::pre_parent_update_set()
						.before(Self::Parent::update_set())
						.after(<Self::Parent as TreePath>::Parent::update_set()),
				);
			} else {
				schedule.configure_set(
					Self::pre_parent_update_set()
						.before(Self::Parent::update_set()),
				);
			}
			schedule.configure_set(
				Self::pre_update_set()
					.before(Self::update_set())
					.after(Self::Parent::update_set()),
			);
			schedule.configure_set(
				Self::update_set().after(Self::Parent::post_update_set()),
			);
			schedule.configure_set(
				Self::post_update_set().after(Self::update_set()),
			);
		} else {
			schedule.configure_set(Self::pre_parent_update_set());
			schedule.configure_set(
				Self::pre_update_set().after(Self::pre_parent_update_set()),
			);
			schedule.configure_set(
				Self::update_set().after(Self::pre_update_set()),
			);
			schedule.configure_set(
				Self::post_update_set().after(Self::update_set()),
			);
		}
	}
}

// all tree paths implement intonodesets
impl<Path: TreePath> IntoNodeSets for Path {}
