use extend::ext;

#[ext]
pub impl f32 {
	fn lerp(start: Self, end: Self, t: Self) -> Self {
		start + (end - start) * t
	}
}
