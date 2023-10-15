use crate::*;
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Score {
	Pass,
	#[default]
	Fail,
	Weight(f32),
	// RankedWeight(u32, f32), TODO
}

pub type ScoreProp<N> = Prop<Score, N>;

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
