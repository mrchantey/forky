use super::u8_shape;



pub const NONE: char = ' ';
pub const VERTICAL: char = '│';
pub const HORIZONTAL: char = '─';
pub const CROSS: char = '┼';

pub const TOP_LEFT: char = '┌';
pub const TOP_RIGHT: char = '┐';
pub const BOTTOM_RIGHT: char = '┘';
pub const BOTTOM_LEFT: char = '└';

pub const TOP_TEE: char = '┬';
pub const BOTTOM_TEE: char = '┴';
pub const LEFT_TEE: char = '├';
pub const RIGHT_TEE: char = '┤';


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
		_ => NONE,
	}
}
