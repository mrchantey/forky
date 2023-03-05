use bevy::{
	prelude::*,
	render::{
		camera::ExtractedCamera,
		render_asset::{self, RenderAssets},
		view::{ExtractedView, ExtractedWindows, WindowSystem},
		RenderApp, RenderSet,
	},
};
use bevy_webxr::*;

fn main() {
	set_panic_hook();
	App::new()
		.add_plugin(demo::SimplePlugin)
		.add_plugin(TexturePlugin)
		.run();
}


pub struct TexturePlugin;


impl Plugin for TexturePlugin {
	fn build(&self, app: &mut App) {
		if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
			render_app.add_system(
				on_render
					.after(WindowSystem::Prepare)
					.in_set(RenderSet::Prepare)
					.after(render_asset::prepare_assets::<Image>),
			);
		}
	}
}


fn on_render(
	// mut commands: Commands,
	_windows: Res<ExtractedWindows>,
	_images: Res<RenderAssets<Image>>,
	// msaa: Res<Msaa>,
	// render_device: Res<RenderDevice>,
	// mut texture_cache: ResMut<TextureCache>,
	_cameras: Query<(Entity, &ExtractedCamera, &ExtractedView)>,
) {
	// for (_entity, camera, _view) in cameras.iter() {
	// 	if let (Some(_target_size), Some(target)) =
	// 		(camera.physical_target_size, &camera.target)
	// 	{
	// 		if let (Some(_out_texture_view), Some(_out_texture_format)) = (
	// 			target.get_texture_view(&windows, &images),
	// 			target.get_texture_format(&windows, &images),
	// 		) {

	// 			let foo = _out_texture_view.take_surface_texture().unwrap();

	// 							// log!("texture: {:?}", out_texture_view);
	// 			log!("here i am");
	// 		}
	// 	}
	// }
}
