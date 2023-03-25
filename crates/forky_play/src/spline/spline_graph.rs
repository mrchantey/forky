use super::*;
use crate::*;
use bevy::prelude::*;
use forky_core::*;
use derive_deref::{Deref, DerefMut};

#[derive(Component, Debug,Deref,DerefMut)]
pub struct SplineNodeComponent<'a>(pub Box<SplineNode<'a>>);


#[derive(Clone, Debug, PartialEq)]
pub struct SplineNode<'a> {
	pub prev: Vec<&'a Box<SplineNode<'a>>>,
	pub next: Vec<&'a Box<SplineNode<'a>>>,
	pub spline: Spline,
}


impl<'a> SplineNode<'a> {
	pub fn new(spline: Spline) -> Box<Self> {
		Box::new(Self {
			prev: Vec::new(),
			next: Vec::new(),
			spline,
		})
	}

	pub fn add_prev(&mut self, node: &'a Box<SplineNode<'a>>) {
		self.prev.push(node);
	}
	pub fn add_next(&mut self, node: &'a Box<SplineNode<'a>>) {
		self.next.push(node);
	}
	pub fn remove_prev(&mut self, node: &'a Box<SplineNode<'a>>) {
		self.prev.remove_first_element(node);
	}
	pub fn remove_next(&mut self, node: &'a Box<SplineNode<'a>>) {
		self.next.remove_first_element(node);
	}


	pub fn get_current_node(&self, t: f32) -> Option<&SplineNode<'a>> {
		if t >= 0.0 && t <= 1.0 {
			return Some(self);
		} else if t < 0.0 && self.prev.len() == 1 {
			return self.prev[0].get_current_node(t + 1.);
		} else if t > 1.0 && self.next.len() == 1 {
			return self.next[0].get_current_node(t - 1.);
		//TODO multiple next/prev
		} else {
			return None;
		}
	}
}
