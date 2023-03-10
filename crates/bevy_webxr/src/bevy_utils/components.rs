use crate::*;
use bevy::prelude::*;
use web_sys::*;



#[derive(Resource)]
pub struct SessionMode(pub web_sys::XrSessionMode);
