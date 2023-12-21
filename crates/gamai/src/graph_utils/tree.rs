use crate::prelude::*;
use petgraph::graph::DiGraph;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

pub struct Tree<T> {
	pub value: T,
	pub children: Vec<Tree<T>>,
}

impl<T: Debug> Debug for Tree<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		if self.children.len() > 0 {
			f.debug_struct("Tree")
				.field("value", &self.value)
				.field("children", &self.children)
				.finish()
		} else {
			f.debug_struct("Tree").field("value", &self.value).finish()
		}
	}
}

impl<T: Clone> Clone for Tree<T> {
	fn clone(&self) -> Self {
		Self {
			value: self.value.clone(),
			children: self.children.clone(),
		}
	}
}

impl<T: PartialEq> PartialEq for Tree<T> {
	fn eq(&self, other: &Self) -> bool {
		self.value == other.value && self.children == other.children
	}
}

impl<T> Tree<T> {
	pub fn new(value: T) -> Self {
		Self {
			value,
			children: Vec::new(),
		}
	}
	pub fn with_child(mut self, child: impl Into<Tree<T>>) -> Self {
		self.children.push(child.into());
		self
	}
	pub fn with_leaf(mut self, child: T) -> Self {
		self.children.push(Tree::new(child));
		self
	}
	pub fn new_with_children(value: T, children: Vec<Self>) -> Self {
		Self { value, children }
	}

	pub fn into_graph(self) -> DiGraph<T, ()> { DiGraph::from_tree(self) }
}


// pub trait IntoTree<T, M> {
// 	fn into_tree(self) -> Tree<T>;
// }
