pub fn quadratic(p0: f32, p1: f32, p2: f32, t: f32) -> f32 {
	p1 + (1.0 - t).powi(2) * (p0 - p1) + t.powi(2) * (p2 - p1)
}

pub fn quadratic_derivative(p0: f32, p1: f32, p2: f32, t: f32) -> f32 {
	2.0 * (1.0 - t) * (p1 - p0) + 2.0 * t * (p2 - p1)
}

pub fn quadratic_derivative2(p0: f32, p1: f32, p2: f32) -> f32 {
	2.0 * (p2 - 2.0 * p1 + p0)
}

pub fn quadratic_derivative3(p0: f32, p1: f32, p2: f32) -> f32 {
	2.0 * (p2 - 3.0 * p1 + 2.0 * p0)
}
