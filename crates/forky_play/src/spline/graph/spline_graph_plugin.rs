use super::*;
use crate::*;
use bevy::prelude::*;

pub struct SplineGraphPlugin;


#[rustfmt::skip]
impl Plugin for SplineGraphPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.init_resource::<SplineGraphLookup>()
			.__();
	}
}


pub struct SplineEcsGraphPlugin;


#[rustfmt::skip]
impl Plugin for SplineEcsGraphPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.insert_resource(EcsSplineGraphLookup::new())
			.add_system(on_ecs_point_moved)
			.__();
	}
}
