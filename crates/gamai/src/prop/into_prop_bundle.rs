use crate::*;
use bevy_ecs::prelude::*;

pub trait IntoBundle {
	fn into_bundle(self) -> impl Bundle;
}
pub trait IntoPropBundle {
	fn into_bundle<Node: AiNode>(self) -> impl Bundle;
}

impl IntoPropBundle for () {
	fn into_bundle<Node: AiNode>(self) -> impl Bundle { () }
}


// for bundle factories
impl<F, B> IntoBundle for F
where
	F: Fn() -> B,
	B: Bundle,
{
	fn into_bundle(self) -> impl Bundle { self() }
}
