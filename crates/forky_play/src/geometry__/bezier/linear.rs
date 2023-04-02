pub fn linear(p0: f32, p1: f32, t: f32) -> f32 { p0 + t * (p1 - p0) }
pub fn linear_derivative(p0: f32, p1: f32) -> f32 { p1 - p0 }
// second & third derivatives are zero
