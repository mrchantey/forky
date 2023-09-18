use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum EdgeState {
	Pass,
	#[default]
	Fail,
	Weight(f32),
	// RankedWeight(u32, f32), TODO
}

impl PartialOrd for EdgeState {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let val = match (self, other) {
			(EdgeState::Fail, EdgeState::Fail) => Ordering::Equal,
			(EdgeState::Fail, _) => Ordering::Less,
			(_, EdgeState::Fail) => Ordering::Greater,
			(EdgeState::Pass, EdgeState::Pass) => Ordering::Equal,
			(EdgeState::Pass, _) => Ordering::Less,
			(_, EdgeState::Pass) => Ordering::Greater,
			(EdgeState::Weight(w1), EdgeState::Weight(w2)) => w1.total_cmp(&w2),
		};
		Some(val)
	}
}