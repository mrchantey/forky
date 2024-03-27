use crate::matchers::*;
use bevy::prelude::*;

// impl<'a> SweetInto<&'a World> for &'a World {
// 	fn sweet_into(self) -> &'a World { &self }
// }
// impl<'a> SweetInto<&'a mut World> for &'a mut World {
// 	fn sweet_into(self) -> &'a mut World { self }
// }

impl<'a> SweetInto<&'a World> for &'a App {
	fn sweet_into(self) -> &'a World { &self.world }
}
impl<'a> SweetInto<&'a mut World> for &'a mut App {
	fn sweet_into(self) -> &'a mut World { &mut self.world }
}
