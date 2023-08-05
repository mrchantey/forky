use super::*;
use crate::*;
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ToolSystemSet {
	Select,
	ModifySelection,
}



#[derive(Default, Clone)]
pub struct ToolPlugin {
	pub settings: InteractionSettings,
}
#[rustfmt::skip]
impl Plugin for ToolPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.init_resource::<CameraRay>()
			.insert_resource(self.settings.clone())
			.add_systems(PreStartup,spawn_resources)
			.configure_set(PreUpdate,ToolSystemSet::Select)
			.configure_set(PreUpdate,ToolSystemSet::ModifySelection
				.after(ToolSystemSet::Select))
			.add_systems(PreUpdate,(
				cast_camera_ray, 
				set_entity_intersect,
				select_entities,
				highlight_entities,
			)
				.chain().in_set(ToolSystemSet::Select))
			.add_systems(PreUpdate,(
				move_selected_interactables,
				set_interactable_colors,
				append_interactable_mesh,
			).in_set(ToolSystemSet::ModifySelection))
			.__();
	}
}
