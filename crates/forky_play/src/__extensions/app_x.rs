use extend::ext;
use bevy::prelude::*;

use crate::utility;

#[ext(name = OptI32X)]
pub impl App{
	fn forky_surrender_focus(&mut self)->&mut Self{
		self.add_startup_system(utility::surrender_focus);
		self
	}
	fn forky_exit_after(&mut self,secs:u64)->&mut Self{
		self.add_system(utility::create_exit_after_system(secs));
		self
	}
}