use super::*;
use bevy_app::prelude::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct AiPlugin<Node, Builder>
where
	Node: AiNode,
	Builder: IntoNode<Node>,
{
	node: Builder,
	phantom: PhantomData<Node>,
}

impl<Node, Builder> AiPlugin<Node, Builder>
where
	Node: AiNode,
	Builder: IntoNode<Node>,
{
	pub fn new(node: Builder) -> Self {
		Self {
			node,
			phantom: PhantomData,
		}
	}
}

impl<Node, Builder> Plugin for AiPlugin<Node, Builder>
where
	Node: AiNode,
	Builder: IntoNode<Node>,
{
	fn build(&self, app: &mut bevy_app::App) {
		app.init_schedule(Update);
		let schedule = app.get_schedule_mut(Update).unwrap();
		self.node.into_node().add_systems(schedule);
	}
}
