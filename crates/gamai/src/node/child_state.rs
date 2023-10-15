use crate::*;
use bevy_ecs::prelude::*;
// use bevy_ecs::query::WorldQuery;
use extend::ext;
use std::marker::PhantomData;
use std::ops::Deref;

/// Tuple used in queries to access child states:
///
/// `(&mut Entity,(&mut Child1,(&mut Child2)))`
pub type ChildIter<T, N> = <N as AiNode>::ChildQuery<T>;
pub type ChildIterOptional<T, N> = <N as AiNode>::ChildQueryOpt<T>;

// #[derive(Clone)]
pub struct ChildState<'a, T: IntoNodeComponent, Parent: AiNode> {
	pub index: usize,
	pub entity: Entity,
	pub value: &'a dyn Deref<Target = T>,
	pub marker: PhantomData<Parent>,
}
pub struct ChildStateOpt<'a, T: IntoNodeComponent, Parent: AiNode> {
	pub index: usize,
	pub entity: Entity,
	pub value: Option<&'a dyn Deref<Target = T>>,
	pub marker: PhantomData<Parent>,
}
pub struct ChildStateMut<'a, T: IntoNodeComponent, Parent: AiNode> {
	pub index: usize,
	pub entity: Entity,
	pub value: Mut<'a, T>,
	pub marker: PhantomData<Parent>,
}
pub struct ChildStateMutRef<'a, T: IntoNodeComponent, Parent: AiNode> {
	pub index: usize,
	pub entity: Entity,
	pub value: &'a mut Mut<'a, T>,
	pub marker: PhantomData<Parent>,
}
pub struct ChildStateOptMut<'a, T: IntoNodeComponent, N: AiNode> {
	pub index: usize,
	pub entity: Entity,
	pub value: Option<Mut<'a, T>>,
	pub marker: PhantomData<N>,
}


pub trait IntoChildState<'a, T> {
	fn index(&self) -> usize;
	fn entity(&self) -> &Entity;
	fn get(&self) -> Option<&T>;
	fn value(&'a mut self) -> &mut Option<Mut<'a, T>>;
	fn set(&mut self, commands: &mut Commands, next: Option<T>);
}

impl<'a, T: IntoNodeComponent, N: AiNode> IntoChildState<'a, T>
	for ChildStateOptMut<'a, T, N>
{
	fn index(&self) -> usize { self.index }
	fn entity(&self) -> &Entity { &self.entity }
	fn get(&self) -> Option<&T> { self.value.as_ref().map(|v| v.deref()) }
	fn value(&'a mut self) -> &mut Option<Mut<'a, T>> { &mut self.value }
	/// Sets child state to given value, with least effort possible.
	/// If both are equal, do nothing
	/// If current is None, use insert command
	/// If next is None, use remove command
	/// If both are Some, mutate self
	fn set(&mut self, commands: &mut Commands, next: Option<T>) {
		match (self.value.as_mut(), next) {
			(None, None) => {
				//noop
			}
			(None, Some(next)) => {
				commands
					.entity(self.entity)
					.insert(NodeComponent::<_, N>::new(next));
			}
			(Some(_), None) => {
				commands.entity(self.entity).remove::<NodeComponent<T, N>>();
			}
			(Some(current), Some(next)) => {
				**current = next;
				// if **current != next {
				// 	**current = next;
				// }
			}
		}
	}
}

pub trait IntoChildQuery<'a, T: IntoNodeComponent, N: AiNode> {
	// type Out;
	// fn out(self) -> Self::Out;
	fn foobar(&self) -> usize { 32 }
}

impl<'a, T: IntoNodeComponent + Clone, N: AiNode> IntoChildQuery<'a, T, N>
	for Vec<ChildStateOptMut<'a, T, N>>
{
}

#[ext]
pub impl<'a, T: IntoNodeComponent + Clone, N: AiNode>
	Vec<Box<dyn IntoChildState<'a, T> + 'a>>
{
}
