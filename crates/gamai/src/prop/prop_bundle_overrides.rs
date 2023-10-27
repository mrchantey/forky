use super::*;
use crate::AiNode;
use bevy_ecs::bundle::Bundle;


pub struct AppendPropOverrides<Node: AiNode, Props: IntoPropBundle> {
	node: Node,
	props: Props,
}

impl<Node: AiNode, Props: IntoPropBundle> AppendPropOverrides<Node, Props> {
	pub fn new(node: Node, props: Props) -> Self { Self { node, props } }
}

impl<Node: AiNode, Props: IntoPropBundle> IntoBundle
	for AppendPropOverrides<Node, Props>
{
	fn into_bundle(self) -> impl Bundle {
		(self.node.into_bundle(), self.props.into_bundle::<Node>())
	}
}

// pub trait IntoBundleWithNode: 'static + Send + Sync {
// 	fn into_bundle<S: IntoPropBundle>(self, system: S) -> impl Bundle;
// }

// pub struct EmptyPropBundleOverrides;

// impl IntoBundleWithNode for EmptyPropBundleOverrides {
// 	fn into_bundle<N: AiNode>(self, node: N) -> impl Bundle {
// 		node.into_bundle()
// 	}
// }

// impl<T: IntoPropBundle + 'static + Send + Sync> IntoBundleWithNode
// 	for PropBundleOverrides<T>
// {
// 	fn into_bundle<S: IntoPropBundle>(self, system: S) -> impl Bundle {
// 		match self {
// 			Self::Replace(value) => value.into_bundle::<S>(),
// 			Self::Append(value) => {
// 				(value.into_bundle::<S>(), system.into_bundle::<S>())
// 			}
// 		}
// 	}
// }
