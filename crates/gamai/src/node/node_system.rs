use crate::*;
use bevy_ecs::prelude::*;

pub trait IntoNodeSystem<M>: 'static + Send + Sync {
	fn into_node_system<Node: AiNode>(
		self,
		schedule: &mut Schedule,
		set: impl SystemSet,
	);
}


// regular bevy systems dont use the generic parameter
impl<T, M> IntoNodeSystem<M> for T
where
	T: IntoSystemConfigs<M> + 'static + Send + Sync,
{
	fn into_node_system<Node: AiNode>(
		self,
		schedule: &mut Schedule,
		set: impl SystemSet,
	) {
		schedule.add_systems(self.in_set(set));
	}
}


// pub struct NodeSystemConfig<T, M> {
// 	system: T,
// 	marker: PhantomData<M>,
// }

// // must be local Into trait
// pub trait IntoNodeSystemConfig<T, M> {
// 	fn into_config(self) -> NodeSystemConfig<T, M>;
// }

// impl<T, M> IntoNodeSystemConfig<T, M> for T
// where
// 	T: IntoSystemConfigs<M>,
// {
// 	fn into_config(self) -> NodeSystemConfig<T, M> {
// 		NodeSystemConfig {
// 			system: self,
// 			marker: PhantomData,
// 		}
// 	}
// }
