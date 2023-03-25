use derive_deref::{Deref, DerefMut};

#[derive(
	Deref, DerefMut, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
pub struct SplineNode(pub u64);
