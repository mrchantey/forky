use crate::*;
use ::bevy::ecs::world::EntityMut;
use ::bevy::prelude::*;
use anyhow::Result;
use extend::ext;

// impl SweetInto<Entity> for &Entity {
// 	fn sweet_into(self) -> Entity { (*self).clone() }
// }
impl SweetInto<Entity> for &EntityMut<'_> {
	fn sweet_into(self) -> Entity { self.id() }
}
impl SweetInto<Entity> for EntityMut<'_> {
	fn sweet_into(self) -> Entity { self.id() }
}

#[ext]
pub impl<'a, W> Matcher<W>
where
	W: 'a + SweetBorrow<&'a World>,
{
	fn to_have_entity(&self, entity: impl SweetInto<Entity>) -> Result<()> {
		let entity = entity.sweet_into();
		let value = self.value.sweet_borrow();
		let received = value.get_entity(entity);
		self.assert_option_with_received_negatable(received)
	}

	fn to_have_component<T: Component>(
		&self,
		entity: impl SweetInto<Entity>,
	) -> Result<()> {
		let entity = entity.sweet_into();
		let received = self.value.sweet_borrow().get::<T>(entity);
		self.assert_option_with_received_negatable(received)
	}

	fn component<T: Component>(
		&self,
		entity: impl SweetInto<Entity>,
	) -> Result<Matcher<&'a T>> {
		let received = self.value.sweet_borrow().get::<T>(entity.sweet_into());
		self.assert_option_with_received(received)
			.map(|c| Matcher::new(c))
	}

	fn to_contain_resource<T: Resource>(&self) -> Result<()> {
		let received = self.value.sweet_borrow().get_resource::<T>();
		self.assert_option_with_received_negatable(received)
	}

	fn resource<T: Resource>(&self) -> Result<Matcher<&'a T>> {
		let received = self.value.sweet_borrow().get_resource::<T>();
		self.assert_option_with_received(received)
			.map(|c| Matcher::new(c))
	}

	fn to_contain_non_send_resource<T: 'static>(&self) -> Result<()> {
		let received = self.value.sweet_borrow().get_non_send_resource::<T>();
		self.assert_option_with_received_negatable(received)
	}

	fn non_send_resource<T: 'static>(&self) -> Result<Matcher<&'a T>> {
		let received = self.value.sweet_borrow().get_non_send_resource::<T>();
		self.assert_option_with_received(received)
			.map(|c| Matcher::new(c))
	}

	//breaks backtracing
	// fn component_to_be<T>(
	// 	&self,
	// 	entity: impl SweetInto<Entity>,
	// 	other: &T,
	// ) -> Result<()>
	// where
	// 	T: Component + PartialEq + std::fmt::Debug,
	// {
	// 	self.component::<T>(entity)?.to_be(other)
	// }
}
