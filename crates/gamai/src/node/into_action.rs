use super::*;
use crate::prop::IntoPropBundle;
use bevy_ecs::all_tuples;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::SystemConfigs;

pub trait AddSystems {
	fn add_systems(self, schedule: &mut Schedule);
}

// impl<T: IntoAction> AddSystems for T {
// 	fn add_systems(self, schedule: &mut Schedule) {
// 		let configs = self.action_into_system_configs::<Self::Node>();
// 		schedule.add_system(configs);
// 	}
// }

// at some point should probably remove this dependency
pub trait IntoAction: IntoPropBundle {
	fn action_into_system_configs<Node: AiNode>(self) -> SystemConfigs;
}

impl IntoAction for () {
	fn action_into_system_configs<Node: AiNode>(self) -> SystemConfigs {
		(|| {}).into_configs()
	}
}

macro_rules! tuples_into_action {
	($($name: ident),*) => {
		impl<$($name: IntoAction),*> IntoAction for ($($name,)*) {
			fn action_into_system_configs<Node: AiNode>(self) -> SystemConfigs {
				#[allow(non_snake_case)]
				let ($($name,)*) = self;
				(
					$($name.action_into_system_configs::<Node>(),)*
				)
				.into_configs()
			}
		}
	}
}

// limit appears to be 12, not sure why
all_tuples!(tuples_into_action, 1, 12, T);
