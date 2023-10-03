use crate::*;
use bevy_ecs::prelude::*;

#[doc(hidden)]
pub struct IsBevySystem;
#[doc(hidden)]
pub struct IsNodeSystem;

pub trait IntoNodeSystem<M>: 'static + Send + Sync {
	fn into_node_system<Node: AiNode>(
		self,
		schedule: &mut Schedule,
		set: impl SystemSet,
	);
}

// functions that return node systems
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
// regular bevy systems that dont use the generic parameter
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
