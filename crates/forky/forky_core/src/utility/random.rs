use rand::Rng;


pub fn random_value() -> f32 { rand::thread_rng().gen_range(0.0..1.0) }
pub fn random_percent_i32() -> i32 { rand::thread_rng().gen_range(0..100) }
