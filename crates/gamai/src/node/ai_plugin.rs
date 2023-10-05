use super::*;
use bevy_app::prelude::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct AiPlugin<Node, Builder>
where
	Node: AiNode,
	Builder: 'static + Send + Sync + Fn() -> Node,
{
	builder: Builder,
	phantom: PhantomData<Node>,
}

impl<Node, Builder> AiPlugin<Node, Builder>
where
	Node: AiNode,
	Builder: 'static + Send + Sync + Fn() -> Node,
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
	Builder: 'static + Send + Sync + Fn() -> Node,
{
	fn build(&self, app: &mut bevy_app::App) {
		app.init_schedule(Update);
		let schedule = app.get_schedule_mut(Update).unwrap();
		(self.builder)().add_systems(schedule);
	}
}
