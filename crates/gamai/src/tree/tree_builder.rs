use crate::*;

pub trait TreeBuilder {
	type Args;
	fn into_element(args: Self::Args) -> impl TreeElement;
}


pub trait DefaultTreeBuilder: TreeBuilder<Args: Default> {
	fn into_default_element() -> impl TreeElement {
		Self::into_element(Default::default())
	}
}

impl<T> DefaultTreeBuilder for T where T: TreeBuilder<Args: Default> {}
