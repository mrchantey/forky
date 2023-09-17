use crate::*;
use bevy::prelude::*;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

// derive debug for `ChoiceActionState` etc
pub trait Choice: std::fmt::Debug + 'static + Send + Sync {
	const INDEX: usize;
	type Agent: Agent;
}

#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct ChoiceActionState<C: Choice> {
	pub state: ActionState,
	marker: PhantomData<C>,
}

impl<C: Choice> Deref for ChoiceActionState<C> {
	type Target = ActionState;
	fn deref(&self) -> &Self::Target { &self.state }
}
impl<C: Choice> DerefMut for ChoiceActionState<C> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state }
}

impl<C: Choice> ChoiceActionState<C> {
	pub fn new(state: ActionState) -> Self {
		Self {
			state,
			marker: PhantomData,
		}
	}
}

#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct ChoiceFactorState<C: Choice> {
	pub state: FactorState,
	marker: PhantomData<C>,
}

impl<C: Choice> ChoiceFactorState<C> {
	pub fn new(state: FactorState) -> Self {
		Self {
			state,
			marker: PhantomData,
		}
	}
}

impl<C: Choice> Deref for ChoiceFactorState<C> {
	type Target = FactorState;
	fn deref(&self) -> &Self::Target { &self.state }
}
impl<C: Choice> DerefMut for ChoiceFactorState<C> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct ChoicePhantom<A: Agent, const INDEX: usize>(PhantomData<A>);

impl<A: Agent, const INDEX: usize> Choice for ChoicePhantom<A, INDEX> {
	type Agent = A;
	const INDEX: usize = INDEX;
}
