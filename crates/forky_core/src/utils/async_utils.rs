use std::sync::{Arc, Mutex};

pub type ArcMut<T> = Arc<Mutex<T>>;
pub fn arcmut<T>(val: T) -> ArcMut<T> { Arc::new(Mutex::new(val)) }


// trait ArcMutex<T> {
// 	fn arcmut(self) -> ArcMut<T> { arcmut(self) }
// }

// impl<T> ArcMutex<T> for T {
// 	// fn arcmut(self) -> ArcMut<T> { arcmut(self) }
// }
