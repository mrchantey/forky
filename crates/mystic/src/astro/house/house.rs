#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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


impl TryFrom<usize> for House {
	type Error = ();

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(House::I),
			1 => Ok(House::II),
			2 => Ok(House::III),
			3 => Ok(House::IV),
			4 => Ok(House::V),
			5 => Ok(House::VI),
			6 => Ok(House::VII),
			7 => Ok(House::VIII),
			8 => Ok(House::IX),
			9 => Ok(House::X),
			10 => Ok(House::XI),
			11 => Ok(House::XII),
			_ => Err(()),
		}
	}
}

impl Into<usize> for &House {
	fn into(self) -> usize { *self as usize }
}
impl Into<usize> for House {
	fn into(self) -> usize { self as usize }
}
