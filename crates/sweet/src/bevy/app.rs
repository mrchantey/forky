use crate::*;
use ::bevy::ecs::world::EntityMut;
use ::bevy::prelude::*;
use anyhow::Result;
use extend::ext;

pub trait IntoEntity {
	fn into_entity(self) -> Entity;
}

impl IntoEntity for Entity {
	fn into_entity(self) -> Entity { self }
}
impl IntoEntity for &Entity {
	fn into_entity(self) -> Entity { self.clone() }
}
impl IntoEntity for &EntityMut<'_> {
	fn into_entity(self) -> Entity { self.id() }
}
impl IntoEntity for EntityMut<'_> {
	fn into_entity(self) -> Entity { self.id() }
}

#[ext]
pub impl Matcher<&App> {
	fn to_have_component<T: Component>(
		&self,
		entity: impl IntoEntity,
	) -> Result<()> {
		let entity = entity.into_entity();
		let received = self.value.world.get::<T>(entity);
		self.assert_option_with_received_negatable(received)
	}

	fn component<T: Component>(
		&self,
		entity: impl IntoEntity,
	) -> Result<Matcher<&T>> {
		let received = self.value.world.get::<T>(entity.into_entity());
		self.assert_option_with_received(received)
			.map(|c| Matcher::new(c))
	}

	fn to_contain_resource<T: Resource>(&self) -> Result<()> {
		let received = self.value.world.get_resource::<T>();
		self.assert_option_with_received_negatable(received)
	}

	fn resource<T: Resource>(&self) -> Result<Matcher<&T>> {
		let received = self.value.world.get_resource::<T>();
		self.assert_option_with_received(received)
			.map(|c| Matcher::new(c))
	}

	fn to_contain_non_send_resource<T: 'static>(&self) -> Result<()> {
		let received = self.value.world.get_non_send_resource::<T>();
		self.assert_option_with_received_negatable(received)
	}

	fn non_send_resource<T: 'static>(&self) -> Result<Matcher<&T>> {
		let received = self.value.world.get_non_send_resource::<T>();
		self.assert_option_with_received(received)
			.map(|c| Matcher::new(c))
	}

	//breaks backtracing
	// fn component_to_be<T>(
	// 	&self,
	// 	entity: impl IntoEntity,
	// 	other: &T,
	// ) -> Result<()>
	// where
	// 	T: Component + PartialEq + std::fmt::Debug,
	// {
	// 	self.component::<T>(entity)?.to_be(other)
	// }
}
