use super::*;
use crate::*;
use bevy_ecs::all_tuples;
use bevy_ecs::prelude::*;

pub trait IntoRawProps {
	fn into_wrapped_bundle<Node: AiNode>(self) -> impl Bundle;
}

pub struct RawProps<T: IntoRawProps>(pub T);

impl<T: IntoRawProps> IntoPropBundle for RawProps<T> {
	fn into_prop_bundle<Node: AiNode>(self) -> impl Bundle {
		self.0.into_wrapped_bundle::<Node>()
	}
}

macro_rules! tuples_into_raw_props {
	($($name: ident),*) => {
		impl<$($name: IntoProp),*> IntoRawProps for ($($name,)*) {
			fn into_wrapped_bundle<Node: AiNode>(self) -> impl Bundle {
				#[allow(non_snake_case)]
				let ($($name,)*) = self;
				(
					$(Prop::<_, Node>::new($name),)*
				)
			}
		}
	}
}

all_tuples!(tuples_into_raw_props, 0, 15, T);
