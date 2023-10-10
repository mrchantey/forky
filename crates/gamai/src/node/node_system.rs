use crate::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::SystemConfigs;

/// not for external use
pub struct IsBevySystem;
/// used for manually declaring an `IntoNodeSystem`
pub struct IsNodeSystem;


/// Node systems are stored in Nodes as `||-> IntoNodeSystem` closures, which also implement `IntoNodeSystem`
pub trait IntoNodeSystem: 'static + Send + Sync + Sized + Default {
	fn into_node_system_configs<Node: AiNode>(self) -> SystemConfigs;
}

impl<T1: IntoNodeSystem, T2: IntoNodeSystem> IntoNodeSystem for (T1, T2) {
	fn into_node_system_configs<Node: AiNode>(self) -> SystemConfigs {
		(
			self.0.into_node_system_configs::<Node>(),
			self.1.into_node_system_configs::<Node>(),
		)
			.into_configs()
	}
}

// // NodeSystem builders, functions that return node systems
// impl<F, T> IntoNodeSystem for F
// where
// 	T: IntoNodeSystem,
// 	F: 'static + Send + Sync + Fn() -> T,
// {
// 	fn into_node_system_configs<Node: AiNode>(self) -> SystemConfigs {
// 		self().into_node_system_configs::<Node>()
// 	}
// }

// 	// fn into_attribute(self) -> Attribute { self().into_attribute() }
// }
// Bevy System Builders
// impl<F, T, M> IntoNodeSystem<(M, IsBevySystem)> for F
// where
// 	T: IntoSystemConfigs<M>,
// 	F: 'static + Send + Sync + Fn() -> T,
// {
// 	fn into_node_system<Node: AiNode>(
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


// impl<T, M> IntoAttribute<M> for T
// where
// 	T: IntoNodeSystem<M>,
// {
// 	fn into_attribute<N: AiNode>(self) -> Attribute {
// 		Attribute::NodeSystem(NodeSystemAttribute {
// 			config: self.configs_from_node::<N>(),
// 		})
// 	}
// }
