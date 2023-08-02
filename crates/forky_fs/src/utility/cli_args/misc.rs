
pub fn get() -> Vec<String> {
	let mut args: Vec<String> = std::env::args().collect();
	args.remove(0);
	args
}
