use crate::*;
use bevy_ecs::prelude::*;

/// not for external use
pub struct IsBevySystem;
/// used for manually declaring an `IntoNodeSystem`
pub struct IsNodeSystem;


/// Node systems are stored in Nodes as `||-> IntoNodeSystem` closures, which also implement `IntoNodeSystem`
pub trait IntoNodeSystem<M>: 'static + Send + Sync {
	fn into_node_system<Node: AiNode>(
		self,
		schedule: &mut Schedule,
		set: impl SystemSet,
	);
}

// NodeSystem builders, functions that return node systems
impl<F, T, M> IntoNodeSystem<(M, IsNodeSystem)> for F
where
	T: IntoNodeSystem<(M, IsNodeSystem)>,
	F: 'static + Send + Sync + Fn() -> T,
{
	fn into_node_system<Node: AiNode>(
		self,
		schedule: &mut Schedule,
		set: impl SystemSet,
	) {
		self().into_node_system::<Node>(schedule, set);
	}
}
// Bevy System Builders
impl<F, T, M> IntoNodeSystem<(M, IsBevySystem)> for F
where
	T: IntoSystemConfigs<M>,
	F: 'static + Send + Sync + Fn() -> T,
{
	fn into_node_system<Node: AiNode>(
		self,
		schedule: &mut Schedule,
		set: impl SystemSet,
	) {
		let system = self();
		schedule.add_systems(system.in_set(set));
	}
}
