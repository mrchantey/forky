use crate::prelude::*;
use petgraph::graph::DiGraph;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GamaiMessage<T: IntoAction + Serialize> {
	SetUpdateSpeed(UpdateSpeed),
	SetGraph(DiGraph<Vec<T>, ()>),
	SetAction(SetActionMessage<T>),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum UpdateSpeed {
	Playing,
	PlayingAtSpeed(f32),
	Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetActionMessage<T: IntoAction> {
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
