use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
	bevy_egui::{self, EguiContext, EguiPlugin},
	bevy_inspector, egui, DefaultInspectorConfigPlugin,
};

pub struct WorldInspectorPlugin;
const DEFAULT_SIZE: (f32, f32) = (320., 160.);

#[derive(Default, Resource)]
pub struct WorldInspectorParams {
	pub enabled: bool,
}


impl Plugin for WorldInspectorPlugin {
	fn build(&self, app: &mut App) {
		if !app.is_plugin_added::<DefaultInspectorConfigPlugin>() {
			app.add_plugin(DefaultInspectorConfigPlugin);
		}
		if !app.is_plugin_added::<EguiPlugin>() {
			app.add_plugin(EguiPlugin);
		}
		app.insert_resource(WorldInspectorParams::default());
		app.add_system(world_inspector_ui);
		app.add_system(toggle_inspector_on_keypress);
	}
}

fn world_inspector_ui(world: &mut World) {
	let params = world.resource::<WorldInspectorParams>();
	if !params.enabled {
		return;
	}
	let mut egui_context = world
		.query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
		.single(world)
		.clone();
	egui::Window::new("World Inspector")
		.default_size(DEFAULT_SIZE)
		.show(egui_context.get_mut(), |ui| {
			egui::ScrollArea::vertical().show(ui, |ui| {
				bevy_inspector::ui_for_world(world, ui);
				ui.allocate_space(ui.available_size());
			});
		});
}

fn toggle_inspector_on_keypress(
	mut params: ResMut<WorldInspectorParams>,
	keys: Res<Input<KeyCode>>,
) {
	if keys.any_just_pressed([KeyCode::G]) {
		params.enabled = !params.enabled;
	}
}

// pub fn toggle_inspector_on_keypress(
// 	mut params: ResMut<WorldInspectorParams>,
// 	keys: Res<Input<KeyCode>>,
// ) {
// 	if keys.any_just_pressed([KeyCode::G]) {
// 		// println!("they removed this feature, dunno how to toggle");
// 		params.enabled = !params.enabled;
// 	}
// }
