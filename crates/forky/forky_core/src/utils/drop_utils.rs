use crate::*;



pub struct DropLogger;

impl Drop for DropLogger {
	fn drop(&mut self) {
		log!("dropped!");
	}
}
