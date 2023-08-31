pub const DEFAULT_EPSILON_F32: f32 = 0.1;
pub const DEFAULT_EPSILON_F64: f64 = 0.1;

pub fn is_close_f32(a: f32, b: f32, epsilon: f32) -> bool {
	abs_diff(a, b) < epsilon
}

pub fn is_close_f64(a: f64, b: f64, epsilon: f64) -> bool {
	abs_diff(a, b) < epsilon
}

pub fn abs_diff<T>(a: T, b: T) -> T
where
	T: PartialOrd + std::ops::Sub<Output = T>,
{
	if a > b {
		a - b
	} else {
		b - a
	}
}
