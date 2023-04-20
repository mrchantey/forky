use strum_macros::{Display, EnumIter};


pub const NUM_MAJOR_ARCANA: u8 = 22;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display)]
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

impl MajorArcana {
	pub fn index(&self) -> u8 { *self as u8 }
}
