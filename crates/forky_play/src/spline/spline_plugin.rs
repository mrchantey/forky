use crate::*;
use bevy::prelude::*;

pub struct SplinePlugin;

impl Plugin for SplinePlugin {
    fn build(&self, app: &mut App) {
        app.__().add_plugins(spline::graph::SplineGraphPlugin).__();
    }
}
