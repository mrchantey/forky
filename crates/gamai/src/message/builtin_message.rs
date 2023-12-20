use crate::prelude::*;
use anyhow::Result;
use bevy_derive::Deref;
use bevy_derive::DerefMut;
use petgraph::graph::NodeIndex;
use serde::Deserialize;
use serde::Serialize;

pub enum UpdateSpeed {
	Playing,
	PlayingAtSpeed(f32),
	Paused,
}

pub enum BuiltinMessage {
	SetUpdateSpeed(UpdateSpeed),
	// LoadTree(TypedNode<BuiltinNodes>),
	SetAction(SetActionMessage),
}


pub trait MessageReader<T> {
	fn read_messages(&mut self, messages: Vec<T>) -> Result<()>;
}
pub trait MessageWriter<T> {
	fn write_messages(&mut self) -> Result<Vec<T>>;
}

#[derive(
	Debug,
	Copy,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Serialize,
	Deserialize,
	Deref,
	DerefMut,
)]
pub struct SerializedNodeIndex(pub usize);

impl From<NodeIndex> for SerializedNodeIndex {
	fn from(index: NodeIndex) -> Self { Self(index.index()) }
}

impl From<SerializedNodeIndex> for NodeIndex {
	fn from(index: SerializedNodeIndex) -> Self { Self::new(index.0) }
}

#[derive(Serialize, Deserialize)]
pub struct SetActionMessage {
	pub index: SerializedNodeIndex,
	pub value: Box<dyn Action>,
}

impl SetActionMessage {
	pub fn new(index: NodeIndex, value: Box<dyn Action>) -> Self {
		Self {
			index: index.into(),
			value,
		}
	}
}
