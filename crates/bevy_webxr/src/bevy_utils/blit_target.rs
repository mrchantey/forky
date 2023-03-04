use bevy::prelude::*;


#[derive(Resource)]
pub struct BlitTarget {
	pub src: Handle<Image>,
	pub dest: wgpu::Texture,
	pub width: u32,
	pub height: u32,
}
