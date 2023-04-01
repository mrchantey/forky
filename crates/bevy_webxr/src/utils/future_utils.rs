



// pub fn block_on<F: Future>(mut future: F) -> F::Output {
// 	// Create a waker that does nothing, since we don't need to be notified of anything
// 	let waker = async_task::waker_fn(|| {});

// 	// Create a context with the waker and default options
// 	let mut context = Context::from_waker(&waker);

// 	// Poll the future until it completes
// 	loop {
// 		match Pin::new(&mut future).poll(&mut context) {
// 			Poll::Ready(output) => return output,
// 			Poll::Pending => {}
// 		}
// 	}
// }
