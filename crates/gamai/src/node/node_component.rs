use crate::*;
use bevy_ecs::prelude::*;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

/// Marker for types that can be used as component fields.
pub trait IntoNodeComponent: 'static + Send + Sync {}
impl<T> IntoNodeComponent for T where T: 'static + Send + Sync {}

pub type DerefProp<T> = dyn Deref<Target = T>;


#[derive(Debug, Clone, Component)]
pub struct NodeComponent<T: IntoNodeComponent, Node: NodeInspector> {
	pub value: T,
	pub marker: PhantomData<Node>,
}


impl<T: IntoNodeComponent, N: NodeInspector> Deref for NodeComponent<T, N> {
	type Target = T;
	fn deref(&self) -> &Self::Target { &self.value }
}
impl<T: IntoNodeComponent, N: NodeInspector> DerefMut for NodeComponent<T, N> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl<T: IntoNodeComponent, N: NodeInspector> NodeComponent<T, N> {
	pub fn new(value: T) -> Self {
		Self {
			value,
			marker: PhantomData,
		}
	}
	pub fn from_node<M>(_n: impl IntoNode<M, Out = N>, value: T) -> Self {
		Self {
			value,
			marker: PhantomData,
		}
	}

	pub fn into_inner(self) -> T { self.value }

	pub fn get<'a, M>(
		_n: impl IntoNode<M, Out = N>,
		world: &'a World,
		entity: Entity,
	) -> Option<&'a T> {
		Self::get_from_node(world, entity)
	}

	pub fn get_ref<'a, M>(
		_n: impl IntoNode<M, Out = N>,
		world: &'a World,
		entity: Entity,
	) -> Option<&'a Self> {
		Self::get_ref_from_node(world, entity)
	}
	pub fn get_from_node<'a>(
		world: &'a World,
		entity: Entity,
	) -> Option<&'a T> {
		world.entity(entity).get::<Self>().map(|v| &v.value)
	}

	pub fn get_ref_from_node<'a>(
		world: &'a World,
		entity: Entity,
	) -> Option<&'a Self> {
		world.entity(entity).get::<Self>()
	}
}



#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Lifecycle {
	#[default]
	Running,
	Success,
	Failure,
}
impl<T: IntoNodeComponent, N: NodeInspector> NodeComponent<T, N>
where
	T: PartialEq,
{
	pub fn set(
		entity: Entity,
		commands: &mut Commands,
		current: Option<&mut Self>,
		next: Option<T>,
	) {
		match (current, next) {
			(None, None) => {
				//noop
			}
			(None, Some(next)) => {
				commands.entity(entity).insert(Self::new(next));
			}
			(Some(_), None) => {
				commands.entity(entity).remove::<Self>();
			}
			(Some(current), Some(next)) => {
				if **current != next {
					**current = next;
				}
			}
		}
	}
}
// impl<'a, T: IntoNodeComponent> NodeComponentRecursive<'a, T> {
// 	pub fn new<N, M>(
// 		n: impl IntoNode<M, Out = N>,
// 		world: &'a World,
// 		entity: Entity,
// 	) -> Self {
// 		n.into_node().get_recursive::<T>(world, entity)
// 	}
// }
