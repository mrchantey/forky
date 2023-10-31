#[allow(unused_imports)]
use crate::node::ActionOrder;
use std::cmp::Ordering;
use std::fmt::Debug;

/// Prop for actions to indicate how favourable they would be to run.
///
/// Usually run in [ActionStage::PreParentUpdate].
///
/// # Example
///
/// ```rust
/// use gamai::tree;
/// use gamai::common_actions::{empty_node,score_always_pass};
///
/// let my_tree = tree!{
/// 	<empty_node before_parent=score_always_pass/>
/// };
///
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Score {
	Pass,
	#[default]
	Fail,
	Weight(f32),
	// RankedWeight(u32, f32), TODO
}

impl Score {
	pub fn set(&mut self, other: Self) { *self = other; }
}

impl PartialOrd for Score {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let val = match (self, other) {
			(Score::Fail, Score::Fail) => Ordering::Equal,
			(Score::Fail, _) => Ordering::Less,
			(_, Score::Fail) => Ordering::Greater,
			(Score::Pass, Score::Pass) => Ordering::Equal,
			(Score::Pass, _) => Ordering::Less,
			(_, Score::Pass) => Ordering::Greater,
			(Score::Weight(w1), Score::Weight(w2)) => w1.total_cmp(&w2),
		};
		Some(val)
	}
}
