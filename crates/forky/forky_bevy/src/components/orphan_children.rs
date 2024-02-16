// use bevy_derive::Deref;
// use bevy_derive::DerefMut;
// use bevy_ecs::prelude::*;
// use bevy_hierarchy::prelude::*;

// /// Container for children that are not in the hierarchy.
// #[derive(Component, Deref, DerefMut)]
// pub struct OrphanChildren(pub Vec<Entity>);

// impl OrphanChildren {
// 	/// does not account for if this orphan has orphan children
// 	pub fn despawn(&self, commands: &mut Commands) {
// 		for child in self.0.iter() {
// 			commands.entity(*child).despawn_recursive();
// 		}
// 	}
// }
