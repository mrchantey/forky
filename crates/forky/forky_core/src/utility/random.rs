use rand::Rng;

/// Returns a random value between 0.0 and 1.0
pub fn random_value() -> f32 { rand::thread_rng().gen_range(0.0..1.0) }
pub fn random_signed() -> f32 { rand::thread_rng().gen_range(-1.0..1.0) }
pub fn random_percent_i32() -> i32 { rand::thread_rng().gen_range(0..100) }
