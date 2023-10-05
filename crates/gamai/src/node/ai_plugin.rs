use super::*;
use bevy_app::prelude::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct AiPlugin<Node, Builder>
where
	Node: AiNode,
	Builder: 'static + Send + Sync + IntoRootNode<Out = Node>,
{
	builder: Builder,
	phantom: PhantomData<Node>,
}

impl<Node, Builder> AiPlugin<Node, Builder>
where
	Node: AiNode,
	Builder: 'static + Send + Sync + IntoRootNode<Out = Node>,
{
	pub fn new(builder: Builder) -> Self {
		Self {
			builder,
			phantom: PhantomData,
		}
	}
}

impl<Node, Builder> Plugin for AiPlugin<Node, Builder>
where
	Node: AiNode,
	Builder: 'static + Send + Sync + Copy + IntoRootNode<Out = Node>,
{
	fn build(&self, app: &mut bevy_app::App) {
		app.init_schedule(Update);
		let schedule = app.get_schedule_mut(Update).unwrap();
		self.builder.into_root_node().add_systems(schedule);
	}
}
