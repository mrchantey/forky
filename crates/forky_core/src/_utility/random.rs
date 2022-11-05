use rand::Rng;


pub fn value() -> f32 { rand::thread_rng().gen_range(0.0..1.0) }
pub fn percent_i32() -> i32 { rand::thread_rng().gen_range(0..100) }
