use crate::*;
use super::*;
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
pub struct ChildProp<'a, T: IntoProp, Parent: AiNode> {
	pub index: usize,
	pub entity: Entity,
	pub value: &'a dyn Deref<Target = T>,
	pub marker: PhantomData<Parent>,
}
pub struct ChildPropOpt<'a, T: IntoProp, Parent: AiNode> {
	pub index: usize,
	pub entity: Entity,
	pub value: Option<&'a dyn Deref<Target = T>>,
	pub marker: PhantomData<Parent>,
}
pub struct ChildPropMut<'a, T: IntoProp, Parent: AiNode> {
	pub index: usize,
	pub entity: Entity,
	pub value: Mut<'a, T>,
	pub marker: PhantomData<Parent>,
}
pub struct ChildPropMutRef<'a, T: IntoProp, Parent: AiNode> {
	pub index: usize,
	pub entity: Entity,
	pub value: &'a mut Mut<'a, T>,
	pub marker: PhantomData<Parent>,
}
pub struct ChildPropOptMut<'a, T: IntoProp, N: AiNode> {
	pub index: usize,
	pub entity: Entity,
	pub value: Option<Mut<'a, T>>,
	pub marker: PhantomData<N>,
}


pub trait IntoChildProp<'a, T> {
	fn index(&self) -> usize;
	fn entity(&self) -> &Entity;
	fn get(&self) -> &'a dyn Deref<Target = T>;
}
impl<'a, T: IntoProp, N: AiNode> IntoChildProp<'a, T> for ChildProp<'a, T, N> {
	fn index(&self) -> usize { self.index }
	fn entity(&self) -> &Entity { &self.entity }
	fn get(&self) -> &'a dyn Deref<Target = T> { self.value }
}

pub trait IntoChildPropOpt<'a, T> {
	fn index(&self) -> usize;
	fn entity(&self) -> &Entity;
	fn get(&self) -> Option<&'a dyn Deref<Target = T>>;
}
impl<'a, T: IntoProp, N: AiNode> IntoChildPropOpt<'a, T>
	for ChildPropOpt<'a, T, N>
{
	fn index(&self) -> usize { self.index }
	fn entity(&self) -> &Entity { &self.entity }
	fn get(&self) -> Option<&'a dyn Deref<Target = T>> { self.value }
}

pub trait IntoChildPropMut<'a, T> {
	fn index(&self) -> usize;
	fn entity(&self) -> &Entity;
	fn get(&self) -> &dyn Deref<Target = T>;
}
impl<'a, T: IntoProp, N: AiNode> IntoChildPropMut<'a, T>
	for ChildPropMut<'a, T, N>
{
	fn index(&self) -> usize { self.index }
	fn entity(&self) -> &Entity { &self.entity }
	fn get(&self) -> &dyn Deref<Target = T> { &self.value }
}

pub trait IntoChildPropOptMut<'a, T> {
	fn index(&self) -> usize;
	fn entity(&self) -> &Entity;
	fn value(&'a mut self) -> &mut Option<Mut<'a, T>>;
	fn get(&self) -> Option<&T>;
	fn set(&mut self, commands: &mut Commands, next: Option<T>);
}

impl<'a, T: IntoProp, N: AiNode> IntoChildPropOptMut<'a, T>
	for ChildPropOptMut<'a, T, N>
{
	fn index(&self) -> usize { self.index }
	fn entity(&self) -> &Entity { &self.entity }
	fn get(&self) -> Option<&T> { self.value.as_ref().map(|v| v.deref()) }
	fn value(&'a mut self) -> &mut Option<Mut<'a, T>> { &mut self.value }
	/// Sets child state to given value, with least effort possible.
	/// If both are [None], do nothing
	/// If current is [None], use insert command
	/// If next is [None], use remove command
	/// If both are [Some], mutate self
	fn set(&mut self, commands: &mut Commands, next: Option<T>) {
		match (self.value.as_mut(), next) {
			(None, None) => {
				//noop
			}
			(None, Some(next)) => {
				commands.entity(self.entity).insert(Prop::<_, N>::new(next));
			}
			(Some(_), None) => {
				commands.entity(self.entity).remove::<Prop<T, N>>();
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

pub trait IntoChildQuery<'a, T: IntoProp, N: AiNode> {
	// type Out;
	// fn out(self) -> Self::Out;
	fn foobar(&self) -> usize { 32 }
}

impl<'a, T: IntoProp + Clone, N: AiNode> IntoChildQuery<'a, T, N>
	for Vec<ChildPropOptMut<'a, T, N>>
{
}

#[ext]
pub impl<'a, T: IntoProp + Clone, N: AiNode>
	Vec<Box<dyn IntoChildPropOptMut<'a, T> + 'a>>
{
}
