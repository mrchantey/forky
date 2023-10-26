use crate::maze::u8_shape;

//https://www.w3.org/TR/xml-entity-names/025.html

pub const NONE: char = ' ';
pub const VERTICAL: char = '│';
pub const HORIZONTAL: char = '─';
pub const VERTICAL_TOP: char = '╵';
pub const VERTICAL_BOTTOM: char = '╷';
pub const HORIZONTAL_LEFT: char = '╴';
pub const HORIZONTAL_RIGHT: char = '╶';


pub const CROSS: char = '┼';

pub const TOP_LEFT: char = '┌';
pub const TOP_RIGHT: char = '┐';
pub const BOTTOM_RIGHT: char = '┘';
pub const BOTTOM_LEFT: char = '└';

pub const TOP_TEE: char = '┬';
pub const BOTTOM_TEE: char = '┴';
pub const LEFT_TEE: char = '├';
pub const RIGHT_TEE: char = '┤';

pub const DIAG_TL_BR: char = '╲';
pub const DIAG_TR_BL: char = '╱';
pub const DIAG_CROSS: char = '╳';


pub fn from_u8(val: u8) -> char {
	match val {
		u8_shape::NONE => NONE,
		u8_shape::VERTICAL => VERTICAL,
		u8_shape::HORIZONTAL => HORIZONTAL,
		u8_shape::CROSS => CROSS,

		u8_shape::TOP_LEFT => TOP_LEFT,
		u8_shape::TOP_RIGHT => TOP_RIGHT,
		u8_shape::BOTTOM_RIGHT => BOTTOM_RIGHT,
		u8_shape::BOTTOM_LEFT => BOTTOM_LEFT,

		u8_shape::TOP_TEE => TOP_TEE,
		u8_shape::BOTTOM_TEE => BOTTOM_TEE,
		u8_shape::LEFT_TEE => LEFT_TEE,
		u8_shape::RIGHT_TEE => RIGHT_TEE,

		u8_shape::HORIZONTAL_LEFT => HORIZONTAL_LEFT,
		u8_shape::HORIZONTAL_RIGHT => HORIZONTAL_RIGHT,
		u8_shape::VERTICAL_TOP => VERTICAL_TOP,
		u8_shape::VERTICAL_BOTTOM => VERTICAL_BOTTOM,

		u8_shape::DIAG_CROSS => DIAG_CROSS,
		u8_shape::DIAG_TL_BR => DIAG_TL_BR,
		u8_shape::DIAG_TR_BL => DIAG_TR_BL,
		_ => NONE,
	}
}
