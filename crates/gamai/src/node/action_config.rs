use super::*;
use crate::common_actions::*;
use crate::prop::IntoPropBundle;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::schedule::SystemConfigs;

// must be generic because `IntoAction` cannot be made into an object
#[derive(Debug, Clone)]
pub struct ActionConfig<A: IntoAction> {
	pub action: A,
	pub apply_deferred: bool,
	pub order: ActionOrder,
	// TODO update timer, pre/post update settings
}

impl<A: IntoAction> IntoActionConfig<A> for ActionConfig<A> {
	fn into_action_config(self) -> ActionConfig<A> { self }
}

pub trait IntoActionConfig<A: IntoAction>: Sized {
	fn into_action_config(self) -> ActionConfig<A>;

	fn apply_deferred(self, val: bool) -> ActionConfig<A> {
		let mut config = self.into_action_config();
		config.apply_deferred = val;
		config
	}
	fn order(self, val: ActionOrder) -> ActionConfig<A> {
		let mut config = self.into_action_config();
		config.order = val;
		config
	}
}

impl IntoActionConfig<()> for () {
	fn into_action_config(self) -> ActionConfig<()> {
		ActionConfig {
			action: (),
			apply_deferred: false,
			order: ActionOrder::default(),
		}
	}
}

impl<A: IntoAction> IntoPropBundle for ActionConfig<A> {
	fn into_prop_bundle<Node: AiNode>(self) -> impl Bundle {
		self.action.into_prop_bundle::<Node>()
	}
}

impl<A: IntoAction> IntoAction for ActionConfig<A> {
	fn action_into_system_configs<Node: AiNode>(self) -> SystemConfigs {
		let action = self.action.action_into_system_configs::<Node>();

		let action = if self.apply_deferred {
			(action, apply_deferred).chain()
		} else {
			action
		}
		.in_set(ActionSet::new::<Node>(self.order));
		(
			action,
			//TODO this should be in the `TreeFirstSet`
			update_action_timer
				.action_into_system_configs::<Node>()
				.in_set(Node::pre_parent_update_set()),
			combined_pre_update
				.action_into_system_configs::<Node>()
				.in_set(Node::pre_update_set()),
			combined_post_update
				.action_into_system_configs::<Node>()
				.in_set(Node::post_update_set()),
		)
			.into_configs()
	}
}


impl<
		T1: IntoActionConfig<A1>,
		T2: IntoActionConfig<A2>,
		A1: IntoAction,
		A2: IntoAction,
	> IntoActionConfig<(A1, A2)> for (T1, T2)
{
	fn into_action_config(self) -> ActionConfig<(A1, A2)> {
		ActionConfig {
			action: (
				self.0.into_action_config().action,
				self.1.into_action_config().action,
			),
			apply_deferred: false,
			order: ActionOrder::default(),
		}
	}
}

// macro_rules! tuples_into_action_config {
// 	($($name: ident),*) => {
// 		//TODO should be name:IntoActionConfig etc
// 		impl<$($name: IntoAction),*> IntoActionConfig<($($name,)*)> for ($($name,)*) {

// 			fn into_action_config(self) -> ActionConfig<($($name,)*)>{
// 				todo!()
// 					// #[allow(non_snake_case)]
// 					// let ($($name,)*) = self;
// 					// (
// 					// 	$($name.action_into_system_configs::<Node>(),)*
// 					// )
// 					// .into_configs()
// 			}
// 			//TODO override order() etc
// 		}
// 	}
// }

// // limit appears to be 12, not sure why
// all_tuples!(tuples_into_action_config, 1, 1, T);
// all_tuples!(tuples_into_action_config, 1, 12, T);
