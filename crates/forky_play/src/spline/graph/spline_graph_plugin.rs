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
