use rand::Rng;


pub fn percent_i32()->i32{
	rand::thread_rng().gen_range(0..100)
}