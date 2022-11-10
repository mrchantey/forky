use std::{rc::Rc, sync::Mutex};

use crate::LedsInterface;

pub struct SketchInterface<'a> {
	leds: Rc<Mutex<dyn LedsInterface + 'a>>,
}


impl<'a> SketchInterface<'a> {
	pub fn new<T>(leds: T) -> SketchInterface<'a>
	where
		T: LedsInterface + 'a,
	{
		SketchInterface {
			leds: Rc::new(Mutex::new(leds)),
		}
	}
}
