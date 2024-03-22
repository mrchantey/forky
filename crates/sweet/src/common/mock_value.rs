use flume::Receiver;
use flume::Sender;
use flume::TryRecvError;



pub fn mock_value<T>() -> MockValue<T> { MockValue::new() }

#[derive(Debug, Clone)]
pub struct MockValue<T> {
	pub send: Sender<T>,
	pub recv: Receiver<T>,
}

impl<T> Default for MockValue<T> {
	fn default() -> Self { Self::new() }
}

impl<T> MockValue<T> {
	pub fn new() -> Self {
		let (send, recv) = flume::unbounded();
		Self { send, recv }
	}
	pub fn push(&self, value: T) {
		self.send
			.send(value)
			.expect("the channel has been disconnected");
	}

	pub fn pop(&self) -> Option<T> {
		match self.recv.try_recv() {
			Ok(value) => Some(value),
			Err(TryRecvError::Empty) => None,
			Err(TryRecvError::Disconnected) => {
				panic!("the channel has been disconnected")
			}
		}
	}
}
