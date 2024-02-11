use super::*;
use crate::*;
use anyhow::Result;
use std::fmt::Debug;

impl<I, O, F> Matcher<&MockFunc<I, O, F>> {
	pub fn to_have_been_called(&self) -> Result<()> {
		let received = self.value.called.lock().unwrap().len();
		self.assert_correct_with_received(
			received > 0,
			&"to have been called",
			&false,
		)
	}
	pub fn to_have_been_called_times(&self, times: usize) -> Result<()> {
		let received = self.value.called.lock().unwrap().len();
		self.assert_correct_with_received(
			received == times,
			&format!("to have been called {times} times"),
			&format!("called {received} times"),
		)
	}
}
impl<I, O: Clone, F> Matcher<&MockFunc<I, O, F>> {
	pub fn nth_return(&self, time: usize) -> Result<Matcher<O>> {
		let vec = self.value.called.lock().unwrap();
		if let Some(received) = vec.get(time) {
			Ok(Matcher::new(received.clone()))
		} else {
			Err(self.to_error_with_received(&"to have been called", &false))
		}
	}
}
impl<I, O: Debug + PartialEq, F> Matcher<&MockFunc<I, O, F>> {
	/// checks the first time it was called
	pub fn to_have_returned_with(&self, expected: &O) -> Result<()> {
		if let Some(received) = self.value.called.lock().unwrap().first() {
			self.assert_correct_with_received(
				received == expected,
				expected,
				received,
			)
		} else {
			Err(self.to_error_with_received(&"to have been called", &false))
		}
	}
	pub fn to_have_returned_nth_with(
		&self,
		time: usize,
		expected: &O,
	) -> Result<()> {
		if let Some(received) = self.value.called.lock().unwrap().get(time) {
			self.assert_correct_with_received(
				received == expected,
				expected,
				received,
			)
		} else {
			Err(self.to_error_with_received(&"to have been called", &false))
		}
	}
}

//TODO to_have_been_called_with
