use super::*;
use bevy_app::prelude::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct AiPlugin<Root>
where
	Root: 'static + Send + Sync + IntoRootNode,
{
	root: Root,
}

impl<Root> AiPlugin<Root>
where
	Root: 'static + Send + Sync + IntoRootNode,
{
	pub fn new(root: Root) -> Self { Self { root } }
}

impl<Builder> Plugin for AiPlugin<Builder>
where
	// Node: AiNode,
	Builder: 'static + Send + Sync + IntoRootNode,
{
	fn build(&self, app: &mut bevy_app::App) {
		app.init_schedule(Update);
		let schedule = app.get_schedule_mut(Update).unwrap();
		self.root.into_root_node().add_systems(schedule);
	}
}
