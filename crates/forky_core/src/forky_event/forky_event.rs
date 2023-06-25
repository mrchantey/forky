use crate::*;

pub type EventListenerUnit = Box<dyn Fn()>;
pub type EventListenerVecUnit = Vec<EventListenerUnit>;
pub type EventListenerVecSendUnit = ArcMut<EventListenerVecUnit>;

pub type EventListener<T> = Box<dyn Fn(&T)>;
pub type EventListenerVec<T> = Vec<EventListener<T>>;
pub type EventListenerVecSend<T> = ArcMut<EventListenerVec<T>>;

#[derive(Default)]
pub struct ForkyEvent<T> {
	pub listeners: Vec<Box<dyn Fn(&T)>>,
}

impl<T> ForkyEvent<T> {
	pub fn new() -> Self {
		Self {
			listeners: Vec::new(),
		}
	}

	pub fn trigger(&self, val: &T) {
		for listener in self.listeners.iter() {
			listener(val);
		}
	}
	pub fn add_listener(&mut self, listener: Box<dyn Fn(&T)>) {
		self.listeners.push(listener);
	}

	pub fn clear(&mut self) { self.listeners.clear(); }
}
