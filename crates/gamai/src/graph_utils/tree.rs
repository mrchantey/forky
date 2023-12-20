use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::{self,};

pub trait IntoTree<T> {
	fn into_tree(self) -> Tree<T>;
	fn with_child(self, child: impl IntoTree<T>) -> Tree<T>;
	fn with_leaf(self, child: T) -> Tree<T>;
}


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


impl<T> IntoTree<T> for Tree<T> {
	fn into_tree(self) -> Tree<T> { self }
	fn with_child(mut self, child: impl IntoTree<T>) -> Tree<T> {
		self.children.push(child.into_tree());
		self
	}
	fn with_leaf(self, child: T) -> Tree<T> {
		self.with_child(Tree::new(child))
	}
}
// impl<T> IntoTree<T> for T {
// 	fn into_tree(self) -> Tree<T> { Tree::new(self) }
// 	fn with_child(self, child: impl IntoTree<T>) -> Tree<T> {
// 		let mut this = self.into_tree();
// 		this.children.push(child.into_tree());
// 		this
// 	}
// }

impl<T> IntoTree<T> for (T, Vec<Tree<T>>) {
	fn into_tree(self) -> Tree<T> {
		Tree {
			value: self.0,
			children: self.1,
		}
	}
	fn with_child(self, child: impl IntoTree<T>) -> Tree<T> {
		let mut this = self.into_tree();
		this.children.push(child.into_tree());
		this
	}
	fn with_leaf(self, child: T) -> Tree<T> {
		let mut this = self.into_tree();
		this.children.push(Tree::new(child));
		this
	}
}

// impl<T> Into<Tree<T>> for (T, Vec<Tree<T>>) {
// 	fn into(self) -> Tree<T> { Tree::<T>::new_with_children(self.0, self.1) }
// }

impl<T> Tree<T> {
	pub fn new(value: T) -> Self {
		Self {
			value,
			children: Vec::new(),
		}
	}
	pub fn with_child(mut self, child: impl IntoTree<T>) -> Self {
		self.children.push(child.into_tree());
		self
	}
	pub fn new_with_children(value: T, children: Vec<Self>) -> Self {
		Self { value, children }
	}
}
