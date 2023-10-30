use crate::node::*;
use crate::prop::*;


pub trait TreeElement: AddSystems + IntoBundle + Sized {
	type Node: AiNode;
	fn into_root(self) -> impl TreeElement { self.with_path::<Self::Node>() }
	fn with_path<NewPath: TreePath>(self) -> impl TreeElement;
}

pub trait IntoElement<M>: Sized {
	type Out: TreeElement;
	type Node: AiNode = <Self::Out as TreeElement>::Node;
	fn into_element(self) -> Self::Out;
}

pub struct IntoElementStruct;
pub struct IntoElementFunc;
pub struct IntoElementPropsFunc;

impl<T> IntoElement<IntoElementStruct> for T
where
	T: TreeElement,
{
	type Out = T;
	fn into_element(self) -> Self::Out { self }
}

impl<F, T> IntoElement<IntoElementFunc> for F
where
	F: Fn() -> T,
	T: TreeElement,
{
	type Out = T;
	fn into_element(self) -> Self::Out { self() }
}
impl<F, P, T> IntoElement<(IntoElementPropsFunc, P)> for F
where
	F: Fn(P) -> T,
	P: Default,
	T: TreeElement,
{
	type Out = T;
	fn into_element(self) -> Self::Out { self(P::default()) }
}
