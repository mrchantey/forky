use crate::*;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy::render::camera::{
	camera_system, CameraProjection, CameraProjectionPlugin,
};
use bevy::render::view::{update_frusta, VisibilitySystems};
use bevy::transform::TransformSystem;

pub struct RawProjectionPlugin;
#[rustfmt::skip]
impl Plugin for RawProjectionPlugin {
	fn build(&self, app: &mut App) {
		app.__()
		.add_plugins(CameraProjectionPlugin::<bevy_utils::RawProjection>::default())
		.add_systems(PostUpdate,update_frusta::<bevy_utils::RawProjection>
    	.after(VisibilitySystems::UpdatePerspectiveFrusta)
			.after(camera_system::<bevy_utils::RawProjection>)
			.after(TransformSystem::TransformPropagate)
			.ambiguous_with(update_frusta::<Projection>)
		)
		.__();
	}
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct RawProjection {
	pub mat: Mat4,
}

impl RawProjection {
	pub fn new(mat: Mat4) -> Self {
		//default perspective near/far
		// mat.z_axis.z = 0.0;
		// mat.w_axis.z = 0.1;
		Self { mat }
	}
	pub fn from_vec_inverted(vec: &Vec<f32>) -> Self {
		//
		let mut mat = Mat4::from_vec(vec);
		mat.y_axis.y *= -1.;
		mat.z_axis.z = 0.; //far: infinite
		mat.w_axis.z = 0.01; //near: 0.01
					 // mat.z_axis.z *= -1.;
					 // mat.w_axis.z *= -1.;
		Self::new(mat)
	}
	pub fn get_near(&self) -> f32 {
		0.01
		// self.mat.w_axis.z / (self.mat.z_axis.z - 1.0)
	}
	pub fn get_far(&self) -> f32 {
		1000.
		// self.mat.w_axis.z / (self.mat.z_axis.z + 1.0)
	}
}

impl Default for RawProjection {
	fn default() -> Self {
		Self {
			//default perspective matrix
			mat: Mat4 {
				x_axis: Vec4::new(2.4142134, 0.0, 0.0, 0.0),
				y_axis: Vec4::new(0.0, 2.4142134, 0.0, 0.0),
				z_axis: Vec4::new(0.0, 0.0, 0.0, -1.0),
				w_axis: Vec4::new(0.0, 0.0, 0.1, 0.0),
			},
		}
	}
}

impl CameraProjection for RawProjection {
	fn get_projection_matrix(&self) -> Mat4 { self.mat }
	fn update(&mut self, _width: f32, _height: f32) {}
	fn far(&self) -> f32 { self.get_far() }
}

/*
0  = z,x = 0,0
1  = z,y = 0,1
2  = z,z = 0,2
3  = z,w = 0,3
4  = y,x = 1,0
5  = y,y = 1,1
6  = y,z = 1,2
7  = y,w = 1,3
8  = z,x = 2,0
9  = z,y = 2,1
10 = z,z = 2,2
11 = z,w = 2,3
12 = w,x = 3,0
13 = w,y = 3,1
14 = w,z = 3,2
15 = w,w = 3,3
*/
/*
PerspectiveProjection Mat4 {
	x_axis: Vec4(2.4142134, 0.0, 0.0, 0.0),
	y_axis: Vec4(0.0, 2.4142134, 0.0, 0.0),
	z_axis: Vec4(0.0, 0.0, 0.0, -1.0),
	w_axis: Vec4(0.0, 0.0, 0.1, 0.0)
}
RawProjection Mat4 {
		x_axis: Vec4(1.4527131, 0.0, 0.0, 0.0),
		y_axis: Vec4(0.0, 1.0, 0.0, 0.0),
		z_axis: Vec4(0.0, 0.0, -1.0002, -1.0),
		w_axis: Vec4(0.0, 0.0, -0.20002, 0.0)
}
*/
