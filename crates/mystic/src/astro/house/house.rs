
#[derive(Default, Debug, Clone, PartialEq)]
pub enum House {
	#[default]
	I,
	II,
	III,
	IV,
	V,
	VI,
	VII,
	VIII,
	IX,
	X,
	XI,
	XII,
}
impl House {
	pub fn to_index(&self) -> usize {
		match self {
			House::I => 0,
			House::II => 1,
			House::III => 2,
			House::IV => 3,
			House::V => 4,
			House::VI => 5,
			House::VII => 6,
			House::VIII => 7,
			House::IX => 8,
			House::X => 9,
			House::XI => 10,
			House::XII => 11,
		}
	}

	pub fn from_index(index: usize) -> Self {
		match index {
			0 => House::I,
			1 => House::II,
			2 => House::III,
			3 => House::IV,
			4 => House::V,
			5 => House::VI,
			6 => House::VII,
			7 => House::VIII,
			8 => House::IX,
			9 => House::X,
			10 => House::XI,
			11 => House::XII,
			_ => panic!("Invalid house index {}", index),
		}
	}
}
