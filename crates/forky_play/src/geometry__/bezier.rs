

#[rustfmt::skip]
pub fn linear(p0: f32, p1: f32, t: f32) -> f32 { 
	p0 + t * (p1 - p0) 
}

pub fn quadratic(p0: f32, p1: f32, p2: f32, t: f32) -> f32 {
	p1 + (1.0 - t).powi(2) * (p0 - p1) + t.powi(2) * (p2 - p1)
}

pub fn cubic(p0: f32, p1: f32, p2: f32, p3: f32, t: f32) -> f32 {
	let it = 1.0 - t;
	it.powi(3) * p0
		+ 3.0 * it.powi(2) * t * p1
		+ 3.0 * it * t.powi(2) * p2
		+ t.powi(3) * p3
}

pub fn tangent_linear(p0: f32, p1: f32) -> f32 {
	p1 - p0
	// t * (p1 - p0)
}

pub fn tangent_quadratic(p0: f32, p1: f32, p2: f32, t: f32) -> f32 {
	2.0 * (1.0 - t) * (p1 - p0) + 2.0 * t * (p2 - p1)
}

pub fn tangent_cubic(p0: f32, p1: f32, p2: f32, p3: f32, t: f32) -> f32 {
	let it = 1.0 - t;
	3.0 * it.powi(2) * (p1 - p0)
		+ 6.0 * it * t * (p2 - p1)
		+ 3.0 * t.powi(2) * (p3 - p2)
}
