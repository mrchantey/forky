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

impl<T: IntoAction> Default for MessagePlugin<T> {
	fn default() -> Self {
		Self {
			phantom: PhantomData,
		}
	}
}

impl<T: 'static + Send + Sync + IntoAction> Plugin for MessagePlugin<T> {
	fn build(&self, app: &mut App) {
		app.insert_resource(Inbox::<T>::default())
			.insert_resource(Outbox::<T>::default())
			.insert_non_send_resource(InboxNonSend::<T>::new())
			.add_systems(
				PreUpdate,
				sync_non_send_inbox::<T>.in_set(MessagePreUpdateSet),
			);
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


#[derive(Clone, Deref, DerefMut)]
pub struct InboxNonSend<T: IntoAction>(pub Arc<Mutex<Inbox<T>>>);
impl<T: IntoAction> InboxNonSend<T> {
	pub fn push(&self, item: GamaiMessage<T>) {
		self.lock().unwrap().push(item);
	}
}

impl<T: IntoAction> InboxNonSend<T> {
	pub fn new() -> Self { Self(Default::default()) }
	pub fn clear(&self) -> Vec<GamaiMessage<T>> {
		std::mem::replace(&mut *self.lock().unwrap(), Vec::new())
	}
}
pub fn sync_non_send_inbox<T: IntoAction>(
	mut inbox: ResMut<Inbox<T>>,
	inbox_non_send: NonSend<InboxNonSend<T>>,
) {
	**inbox = inbox_non_send.clear();
}
