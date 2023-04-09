use bevy::prelude::*;

#[derive(Default, Component, Deref, DerefMut)]
pub struct SplineVelocity(pub f32);

#[derive(Default, Component, Deref, DerefMut)]
pub struct SplinePosition(pub f32);
