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
