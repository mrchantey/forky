use super::*;
use bevy_app::prelude::*;

#[derive(Debug, Clone)]
pub struct AiPlugin<Node>
where
	Node: AiNode,
	// Builder: 'static + Send + Sync + IntoRootNode<Out = Node>,
{
	node: Node, // builder: Builder,
	            // phantom: PhantomData<Node>,
}

impl<Node> AiPlugin<Node>
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

impl<Node> Plugin for AiPlugin<Node>
where
	Node: AiNode,
{
	fn build(&self, app: &mut bevy_app::App) {
		app.init_schedule(Update);
		let schedule = app.get_schedule_mut(Update).unwrap();
		self.node.clone().add_systems(schedule);
	}
}
