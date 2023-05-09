use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};


pub const NUM_MAJOR_ARCANA: u8 = 22;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display, Hash)]
#[strum(serialize_all = "title_case")]
pub enum MajorArcana {
	// #[strum(serialize = "The Fool")]
	TheFool,
	// #[strum(serialize = "The Magician")]
	TheMagician,
	// #[strum(serialize = "The High Priestess")]
	TheHighPriestess,
	// #[strum(serialize = "The Empress")]
	TheEmpress,
	// #[strum(serialize = "The Emperor")]
	TheEmperor,
	// #[strum(serialize = "The Hierophant")]
	TheHierophant,
	// #[strum(serialize = "The Lovers")]
	TheLovers,
	// #[strum(serialize = "The Chariot")]
	TheChariot,
	Strength,
	// #[strum(serialize = "The Hermit")]
	TheHermit,
	// #[strum(serialize = "The Wheel of Fortune")]
	TheWheelOfFortune,
	Justice,
	#[strum(serialize = "The Hanged Man")]
	TheHangedMan,
	Death,
	Temperance,
	#[strum(serialize = "The Devil")]
	TheDevil,
	#[strum(serialize = "The Tower")]
	TheTower,
	#[strum(serialize = "The Star")]
	TheStar,
	#[strum(serialize = "The Moon")]
	TheMoon,
	#[strum(serialize = "The Sun")]
	TheSun,
	Judgement,
	#[strum(serialize = "The World")]
	TheWorld,
}

impl Into<usize> for &MajorArcana {
	fn into(self) -> usize { *self as usize }
}
impl Into<usize> for MajorArcana {
	fn into(self) -> usize { self as usize }
}

impl TryFrom<usize> for MajorArcana {
	type Error = ();
	fn try_from(value: usize) -> Result<Self, Self::Error> {
		Self::iter().nth(value).ok_or(())
	}
}
