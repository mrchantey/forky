use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

pub type ArcMut<T> = Arc<Mutex<T>>;
pub fn arcmut<T>(val: T) -> ArcMut<T> { Arc::new(Mutex::new(val)) }

pub type RcCell<T> = Rc<RefCell<T>>;
pub fn rccell<T>(val: T) -> RcCell<T> { Rc::new(RefCell::new(val)) }


pub async fn retry_async<T, E, F>(
	func: impl Fn() -> F,
	timeout: Duration,
) -> Result<T, E>
where
	F: Future<Output = Result<T, E>>,
{
	let start = std::time::Instant::now();
	loop {
		match func().await {
			Ok(val) => return Ok(val),
			Err(err) => {
				if start.elapsed() > timeout {
					return Err(err);
				}
			}
		}
		std::thread::sleep(Duration::from_millis(10));
	}
}


// trait ArcMutex<T> {
// 	fn arcmut(self) -> ArcMut<T> { arcmut(self) }
// }

// impl<T> ArcMutex<T> for T {
// 	// fn arcmut(self) -> ArcMut<T> { arcmut(self) }
// }
