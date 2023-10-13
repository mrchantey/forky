use crate::*;
use bevy_ecs::prelude::*;
use std::marker::PhantomData;

pub trait NodeComponent {
	type Value<Node: NodeInspector>: Component;

	fn get<'a, Node: NodeInspector>(
		_n: &Node,
		world: &'a World,
		entity: Entity,
	) -> Option<&'a Self::Value<Node>> {
		world.entity(entity).get::<Self::Value<Node>>()
	}
}


pub struct NodeComponentRecursive<'a, T: NodeComponent, N: NodeInspector> {
	pub value: Option<&'a T::Value<N>>,
	pub children: Vec<NodeComponentRecursive<'a, T, N>>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Lifecycle {
	#[default]
	Running,
	Success,
	Failure,
}

#[derive(Debug, Clone, Component, PartialEq)]
pub struct LifecycleWrapper<N: NodeInspector> {
	pub value: Lifecycle,
	pub marker: PhantomData<N>,
}

impl NodeComponent for Lifecycle {
	type Value<Node: NodeInspector> = LifecycleWrapper<Node>;
}



// impl<'a, T: NodeComponent, N: NodeInspector> NodeComponentRecursive<'a, T, N> {
// 	pub fn new(n: N, world: &'a World, entity: Entity) -> Self {
// 		Self {
// 			value: T::get(world, entity),
// 			children: n
// 				.get_children()
// 				.into_iter()
// 				.map(|c| Self::new(*c, world, entity))
// 				.collect(),
// 		}
// 	}
// }
