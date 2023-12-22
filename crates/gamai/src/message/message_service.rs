use crate::prelude::*;
use bevy_app::prelude::*;
use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct MessagePostUpdateSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct MessagePreUpdateSet;

pub struct MessagePlugin<T: IntoAction> {
	phantom: PhantomData<T>,
}

impl<T: 'static + Send + Sync + IntoAction> Plugin for MessagePlugin<T> {
	fn build(&self, app: &mut App) {
		app.insert_resource(Inbox::<T>::default())
			.insert_resource(Outbox::<T>::default())
			.add_systems(
				PreUpdate,
				collect_inbox::<T>.in_set(MessagePreUpdateSet),
			)
			.add_systems(
				PostUpdate,
				collect_outbox::<T>.in_set(MessagePostUpdateSet),
			);
	}
}



#[derive(Default)]
pub struct MessageService<T: IntoAction> {
	/// Messages received
	pub inbox: Arc<Mutex<Vec<GamaiMessage<T>>>>,
	/// Messages pending send
	pub outbox: Arc<Mutex<Vec<GamaiMessage<T>>>>,
}


impl<T: IntoAction> MessageService<T> {
	pub fn new() -> Self {
		Self {
			inbox: Default::default(),
			outbox: Default::default(),
		}
	}
	pub fn take_inbox(&self) -> Vec<GamaiMessage<T>> { Self::take(&self.inbox) }

	pub fn take_outbox(&self) -> Vec<GamaiMessage<T>> {
		Self::take(&self.outbox)
	}
	pub fn push_outbox(&self, item: GamaiMessage<T>) {
		self.outbox.lock().unwrap().push(item);
	}
	pub fn extend_outbox(&self, items: Vec<GamaiMessage<T>>) {
		self.outbox.lock().unwrap().extend(items);
	}


	fn take(list: &Arc<Mutex<Vec<GamaiMessage<T>>>>) -> Vec<GamaiMessage<T>> {
		let mut list = list.lock().unwrap();
		std::mem::replace(&mut *list, Vec::new())
	}
}

#[derive(Clone, Serialize, Deserialize, Resource, Deref, DerefMut)]
pub struct Inbox<T: IntoAction>(pub Vec<GamaiMessage<T>>);
#[derive(Clone, Serialize, Deserialize, Resource, Deref, DerefMut)]
pub struct Outbox<T: IntoAction>(pub Vec<GamaiMessage<T>>);
impl<T: IntoAction> Default for Inbox<T> {
	fn default() -> Self { Self(Vec::new()) }
}
impl<T: IntoAction> Default for Outbox<T> {
	fn default() -> Self { Self(Vec::new()) }
}


fn collect_inbox<T: IntoAction>(
	mut inbox: ResMut<Inbox<T>>,
	service: NonSend<MessageService<T>>,
) {
	**inbox = service.take_inbox();
}

fn collect_outbox<T: IntoAction>(
	mut outbox: ResMut<Outbox<T>>,
	service: NonSend<MessageService<T>>,
) {
	service.extend_outbox(std::mem::replace(&mut **outbox, Vec::new()));
}
