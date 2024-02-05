use crate::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Deref;
use std::ops::DerefMut;

pub trait ActionSuper: Action + PartialEq {}
impl<T: Action + PartialEq> ActionSuper for T {}


#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct BehaviorNode<T: Action> {
	pub name: Option<String>,
	pub actions: Vec<T>,
}

impl<T: Action> Into<BehaviorNode<T>> for Vec<T> {
	fn into(self) -> BehaviorNode<T> {
		BehaviorNode {
			actions: self,
			name: None,
		}
	}
}


impl<T: Action> Deref for BehaviorNode<T> {
	type Target = Vec<T>;
	fn deref(&self) -> &Self::Target { &self.actions }
}
impl<T: Action> DerefMut for BehaviorNode<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.actions }
}


impl<T: Action> BehaviorNode<T> {
	pub fn new(actions: Vec<T>) -> Self {
		Self {
			name: None,
			actions,
		}
	}
}
