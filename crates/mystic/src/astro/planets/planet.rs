use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumCount, EnumIter)]
pub enum Planet {
	Sun,
	Moon,
	Mercury,
	Venus,
	Earth,
	Mars,
	Jupiter,
	Saturn,
	Uranus,
	Neptune,
	Pluto,
}

impl TryFrom<usize> for Planet {
	type Error = ();

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		Self::iter().nth(value).ok_or(())
	}
}
