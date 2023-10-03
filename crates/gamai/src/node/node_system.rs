use crate::*;
use bevy_ecs::prelude::*;

pub trait IntoNodeSystem<M>: 'static + Send + Sync {
	fn into_node_system<Node: AiNode>(
		self,
		schedule: &mut Schedule,
		set: impl SystemSet,
	);
}

pub trait NodeSystemBuilder<M> {
	fn get_system<Node: AiNode>(self) -> impl IntoSystemConfigs<M>;
}

// regular bevy systems that dont use the generic parameter
impl<T, M> NodeSystemBuilder<M> for T
where
	T: IntoSystemConfigs<M>,
{
	fn get_system<Node: AiNode>(self) -> impl IntoSystemConfigs<M> { self }
}


impl<F, T, M> IntoNodeSystem<M> for F
where
	T: NodeSystemBuilder<M>,
	F: 'static + Send + Sync + Fn() -> T,
// impl<F, T, M> IntoNodeSystem<M> for F
// where
// 	T: NodeSystemBuilder<M>,
// 	F: 'static + Send + Sync + Fn() -> T,
{
	fn into_node_system<Node: AiNode>(
		self,
		schedule: &mut Schedule,
		set: impl SystemSet,
	) {
		let system = self().get_system::<Node>();
		schedule.add_systems(system.in_set(set));
	}
}
