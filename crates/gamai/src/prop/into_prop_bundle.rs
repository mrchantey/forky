use crate::*;
use bevy_ecs::all_tuples;
use bevy_ecs::prelude::*;

pub trait IntoBundle {
	fn into_bundle(self) -> impl Bundle;
}

pub trait IntoPropBundle {
	fn into_prop_bundle<Node: AiNode>(self) -> impl Bundle;
}

macro_rules! tuples_into_prop_bundle {
	($($name: ident),*) => {
		impl<$($name: IntoPropBundle),*> IntoPropBundle for ($($name,)*) {
			fn into_prop_bundle<Node: AiNode>(self) -> impl Bundle {
				#[allow(non_snake_case)]
				let ($($name,)*) = self;
				(
					$($name.into_prop_bundle::<Node>(),)*
				)
			}
		}
	}
}
all_tuples!(tuples_into_prop_bundle, 0, 15, T);

// impl<T> IntoPropBundle for T
// where
// 	T: IntoBundle,
// {
// 	fn into_prop_bundle<Node: AiNode>(self) -> impl Bundle {
// 		self.into_bundle()
// 	}
// }

// impl<F, B> IntoBundle for F
// where
// 	F: Fn() -> B,
// 	B: Bundle,
// {
// 	fn into_bundle(self) -> impl Bundle { self() }
// }
