use std::cell::RefCell;

use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
// use std::time::Duration;



pub type ArcMut<T> = Arc<Mutex<T>>;
pub fn arcmut<T>(val: T) -> ArcMut<T> { Arc::new(Mutex::new(val)) }

pub type RcCell<T> = Rc<RefCell<T>>;
pub fn rccell<T>(val: T) -> RcCell<T> { Rc::new(RefCell::new(val)) }



// trait ArcMutex<T> {
// 	fn arcmut(self) -> ArcMut<T> { arcmut(self) }
// }

// impl<T> ArcMutex<T> for T {
// 	// fn arcmut(self) -> ArcMut<T> { arcmut(self) }
// }
