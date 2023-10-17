use super::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::SystemConfigs;
use std::fmt::Debug;
use std::hash::Hash;

/// Marker type that tells attributes to ignore that system
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct EmptyAction;

impl IntoAction for EmptyAction {
	const IS_EMPTY: bool = true;
	fn into_action_configs<Node: AiNode>(self) -> SystemConfigs {
		(|| {}).into_configs()
	}
}
/// Node systems are stored in Nodes as `||-> IntoAction` closures, which also implement `IntoAction`
pub trait IntoAction:
	'static + Send + Sync + Sized + Eq + Hash + Clone + Debug
{
	const IS_EMPTY: bool = false;
	fn into_action_configs<Node: AiNode>(self) -> SystemConfigs;
}

impl<T1: IntoAction, T2: IntoAction> IntoAction for (T1, T2) {
	fn into_action_configs<Node: AiNode>(self) -> SystemConfigs {
		(
			self.0.into_action_configs::<Node>(),
			self.1.into_action_configs::<Node>(),
		)
			.into_configs()
	}
}
// impl<T: IntoSystemConfigs<M>, M> IntoAction for T {
// 	fn into_action_configs<Node: AiNode>(self) -> SystemConfigs {
// 		self.into_configs()
// 	}
// }

// // Action builders, functions that return actions
// impl<F, T> IntoAction for F
// where
// 	T: IntoAction,
// 	F: 'static + Send + Sync + Fn() -> T,
// {
// 	fn into_action_configs<Node: AiNode>(self) -> SystemConfigs {
// 		self().into_action_configs::<Node>()
// 	}
// }

// 	// fn into_attribute(self) -> Attribute { self().into_attribute() }
// }
// Bevy System Builders
// impl<F, T, M> IntoAction<(M, IsBevySystem)> for F
// where
// 	T: IntoSystemConfigs<M>,
// 	F: 'static + Send + Sync + Fn() -> T,
// {
// 	fn into_action<Node: AiNode>(
// 		self,
// 		schedule: &mut Schedule,
// 		set: impl SystemSet,
// 	) {
// 		let system = self();
// 		schedule.add_systems(system.in_set(set));
// 	}
// 	//bevy systems ignore the node type
// 	fn configs_from_node<N: AiNode>(self) -> SystemConfigs {
// 		self().into_configs()
// 	}
// }
