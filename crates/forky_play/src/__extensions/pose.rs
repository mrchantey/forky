use bevy::prelude::*;


#[derive(Debug)]
pub struct Pose {
	pub position: Vec3,
	pub rotation: Quat,
}

impl Default for Pose {
	fn default() -> Self {
		Pose {
			position: Vec3::default(),
			rotation: Quat::default(),
		}
	}
}


impl Pose {
	pub fn from_transform(tran: &Transform) -> Pose {
		Pose {
			position: tran.translation,
			rotation: tran.rotation,
		}
	}
	pub fn set_from_transform(&mut self, tran: &Transform) {
		self.position = tran.translation;
		self.rotation = tran.rotation;
	}
}
