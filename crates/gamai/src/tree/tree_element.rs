use crate::node::*;
use crate::prop::*;
use bevy_ecs::prelude::*;
// use crate::*;

/// Track the node, props and action of an element. In the case where no
/// additional props are added, the `props` and `action` will be the same.
#[derive(Debug, Clone, Copy)]
pub struct TreeElement<Node: AiNode, Props: IntoPropBundle, Action: IntoAction>
{
	pub node: Node,
	pub props: Props,
	pub action: Action,
}

impl<Node: AiNode, Props: IntoPropBundle, Action: IntoAction>
	TreeElement<Node, Props, Action>
{
	pub fn new(node: Node, props: Props, action: Action) -> Self {
		Self {
			node,
			props,
			action,
		}
	}
}

// impl IntoBundle


pub trait IntoTreeElement {
	fn add_systems(self, schedule: &mut Schedule);
	fn into_bundle(self) -> impl Bundle;
}
