use bevy_ecs::component::ComponentId;
use bevy_ecs::component::ComponentInfo;
use bevy_ecs::prelude::*;
use extend::ext;

#[ext(name=CoreWorldExtSweet)]
/// Ease-of-use extensions for `bevy::World`
pub impl World {
	fn all_component_ids<'a>(
		&'a self,
		entity: Entity,
	) -> impl Iterator<Item = ComponentId> + 'a {
		for archetype in self.archetypes().iter() {
			if archetype.entities().iter().any(|e| e.entity() == entity) {
				return archetype.components();
			}
		}
		self.archetypes().empty().components()
	}
	fn all_component_infos<'a>(
		&'a self,
		entity: Entity,
	) -> impl Iterator<Item = &'a ComponentInfo> + 'a {
		let components = self.components();
		self.all_component_ids(entity).map(|id| {
			components
				.get_info(id)
				.expect("Component Id without info, this shouldnt happen..")
		})
	}
}
