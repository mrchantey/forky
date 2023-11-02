use super::*;
use crate::tree::IntoElement;
use bevy_app::prelude::*;
use std::marker::PhantomData;

/// A plugin that adds all systems in a tree to the app's `Update` schedule.
#[derive(Debug, Clone)]
pub struct TreePlugin<F, M>
where
	M: 'static + Send + Sync,
	F: IntoElement<M> + Clone + 'static + Send + Sync,
{
	phantom: PhantomData<M>,
	element: F,
}

impl<F, M> TreePlugin<F, M>
where
	M: 'static + Send + Sync,
	F: IntoElement<M> + Clone + 'static + Send + Sync,
{
	pub fn new(into_element: F) -> Self {
		Self {
			element: into_element,
			phantom: PhantomData,
		}
	}
}

impl<F, M> Plugin for TreePlugin<F, M>
where
	M: 'static + Send + Sync,
	F: IntoElement<M> + Clone + 'static + Send + Sync,
{
	fn build(&self, app: &mut bevy_app::App) {
		app.init_schedule(Update);
		let schedule = app.get_schedule_mut(Update).unwrap();
		self.element
			.clone()
			.into_element()
			.add_systems(schedule);
	}
}
