use std::fmt::Debug;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
	#[default]
	Once,
	Repeating,
	//yield? as in happy to continue but check edges again
}

impl NodeState {
	pub fn finish(&mut self) { *self = NodeState::Once; }
}
