use crate::matchers::*;
use ::bevy_app::prelude::*;
use ::bevy_ecs::prelude::*;


impl<'a> SweetInto<&'a World> for &'a App
{ fn sweet_into(self) -> &'a World { &self.world } }
