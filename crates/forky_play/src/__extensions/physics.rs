use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use extend::ext;
use forky_core::{math::*, *};
use std::cmp;

use crate::utility;

#[ext]
pub impl AdditionalMassProperties {
	//TODO forward faces back
	fn one() -> AdditionalMassProperties {
		AdditionalMassProperties::MassProperties(MassProperties {
			principal_inertia: Vec3::ONE,
			mass: 1.,
			..default()
		})
	}
}
#[ext]
pub impl RapierConfiguration {
	//TODO forward faces back
	fn with_gravity_scalar(gravity: f32) -> RapierConfiguration {
		RapierConfiguration {
			gravity: Vec3::from_y(GRAVITY * gravity),
			..default()
		}
	}
}
