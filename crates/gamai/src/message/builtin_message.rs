use crate::prelude::*;
use petgraph::graph::DiGraph;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Clone, Serialize, Deserialize)]
pub enum GamaiMessage<T: IntoAction> {
	/// Used for displaying a message to user via terminal, ui etc.
	Text(String),
	SetUpdateSpeed(UpdateSpeedMessage),
	/// Spawn or respawn the entity with a new graph.
	SetGraph(DiGraph<Vec<T>, ()>),
	/// Updates the value of a specific action on the graph.
	SetAction(SetActionMessage<T>),
}

impl<T: IntoAction> Debug for GamaiMessage<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&serde_json::to_string(&self).unwrap())
	}
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum UpdateSpeedMessage {
	Playing,
	PlayingAtSpeed(f32),
	Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetActionMessage<T: IntoAction> {
	/// TODO specify target entity index
	pub value: T,
	pub node_index: usize,
	pub vec_index: usize,
}


impl<T: IntoAction> SetActionMessage<T> {
	pub fn new(value: T, node_index: usize, vec_index: usize) -> Self {
		Self {
			value,
			node_index,
			vec_index,
		}
	}
}
// pub trait MessageReader<T: IntoAction> {
// 	fn read_messages(&mut self, messages: Vec<GamaiMessage<T>>) -> Result<()>;
// }
// pub trait MessageWriter<T: IntoAction> {
// 	fn write_messages(&mut self) -> Result<Vec<GamaiMessage<T>>>;
// }
