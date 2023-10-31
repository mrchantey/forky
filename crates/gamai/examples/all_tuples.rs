use bevy_ecs::all_tuples;


pub fn main() {}


// impl<T1: IntoAction, T2: IntoAction> IntoAction for (T1, T2) {
// 	fn into_action_configs<Node: AiNode>(self) -> SystemConfigs {
// 		(
// 			self.0.into_action_configs::<Node>(),
// 			self.1.into_action_configs::<Node>(),
// 		)
// 			.into_configs()
// 	}
// }



// macro_rules! impl_plugins_tuples {
// 	($(($param: ident)),*) => {
// 			// impl<$($param),*> IntoAction for ($($param,)*)
// 			// where
// 			// 		$($param: IntoAction),*
// 			// {
// 			// fn into_action_configs<Node: AiNode>(self) -> SystemConfigs {
// 			// (
// 			// 	let ($($plugins,)*) = self;
// 			// 	$(self.$plugins.into_action_configs::<Node>(),)*
// 			// )
// 			// .into_configs()
// 			// }
// 		// }
// 	}
// }

macro_rules! tuple_impl {
	($($name: ident),*) => {
		impl<$($name: IntoAction),*> IntoAction for ($($name,)*) {
			fn into_action_configs<Node: AiNode>(self) -> SystemConfigs {
				let ($($name,)*) = self;
				(
					$($name.into_action_configs::<Node>(),)*
				)
				.into_configs()
			}
		}
	}
}

all_tuples!(tuple_impl, 0, 15, T);
// all_tuples!(impl_plugins_tuples, 0, 15, P, S);
