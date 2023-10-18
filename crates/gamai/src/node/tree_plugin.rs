use super::*;
use bevy_app::prelude::*;

/// A plugin that adds all systems in a tree to the app's `Update` schedule.
#[derive(Debug, Clone)]
pub struct TreePlugin<Node>
where
	Node: AiNode,
{
	node: Node,
}

impl<Node> TreePlugin<Node>
where
	Node: AiNode,
	// Builder: 'static + Send + Sync + IntoRootNode<Out = Node>,
{
	pub fn new<M>(node: impl IntoNode<M, Out = Node>) -> Self {
		Self {
			node: node.into_node(),
		}
	}
}

impl<Node> Plugin for TreePlugin<Node>
where
	Node: AiNode,
{
	fn build(&self, app: &mut bevy_app::App) {
		app.init_schedule(Update);
		let schedule = app.get_schedule_mut(Update).unwrap();
		self.node.clone().add_systems(schedule);
	}
}
