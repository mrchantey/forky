use std::{cmp::Ordering, ops::DerefMut};
use std::fmt::Debug;
use bevy::prelude::*;

/// Alias for DerefMut<Target = FactorState>
pub trait DerefFactorState:
	Debug + Default + DerefMut<Target = FactorState> + Component
{
}
impl<T> DerefFactorState for T where
	T: Debug + Default + DerefMut<Target = FactorState> + Component
{
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum FactorState {
	Pass,
	#[default]
	Fail,
	Weight(f32),
	// RankedWeight(u32, f32), TODO, adds complexity
}



// pub trait Factor {
// 	fn run(&self, props: &mut Props) -> FactorResult;
// }
// impl<T> Factor for T
// where
// 	T: Fn(&mut Props) -> FactorResult,
// {
// 	fn run(&self, props: &mut Props) -> FactorResult { self(props) }
// }

// pub struct FactorIter(pub Vec<Box<dyn Factor>>);

// impl Factor for FactorIter {
// 	fn run(&self, props: &mut Props) -> FactorResult {
// 		self.0.iter().map(|factor| factor.run(props)).fold_factors()
// 	}
// }
impl PartialOrd for FactorState {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let val = match (self, other) {
			(FactorState::Fail,FactorState::Fail)=>Ordering::Equal,
			(FactorState::Fail,_)=>Ordering::Less,
			(_,FactorState::Fail)=>Ordering::Greater,
			(FactorState::Pass, FactorState::Pass) => Ordering::Equal,
			(FactorState::Pass, _) => Ordering::Less,
			(_, FactorState::Pass) => Ordering::Greater,
			(FactorState::Weight(w1), FactorState::Weight(w2)) => {
				w1.total_cmp(&w2)
			}
		};
		Some(val)
	}
}

// #[ext(name=FactorIterExt)]
// pub impl<I> I
// where
// 	I: Iterator<Item = FactorState>,
// {
// 	fn fold_factors(self) -> FactorState {
// 		self.fold(FactorState::Pass, |prev, next|
// 			//TODO refactor into tuple match
// 			match (prev,next){
// 				//if either veto, then veto
// 				(FactorState::Fail,_) => FactorState::Fail,
// 				(_,FactorState::Fail) => FactorState::Fail,
// 				//else if either pass, then pass
// 				(_,FactorState::Pass)=>FactorState::Pass,
// 				(FactorState::Pass,_)=>FactorState::Pass,
// 				//else (somehow?) combine weights
// 				(FactorState::Weight(prev_weight),FactorState::Weight(next_weight)) => 
// 					FactorState::Weight(prev_weight+next_weight),				
// 			}
// 		)
// 	}
// }