use bevy::prelude::*;
use crate::*;
use super::*;

pub struct SplinePlugin;

impl Plugin for SplinePlugin {
	fn build(&self, app: &mut App) { 
		app.__()
    .init_resource::<SplineGraph>()			
			.__(); 
		}
}