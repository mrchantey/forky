use strum_macros::{EnumCount};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumCount)]
pub enum Body {
	Sun,
	Mercury,
	Venus,
	Earth,
	Moon,
	Mars,
	Jupiter,
	Saturn,
	Uranus,
	Neptune,
}
