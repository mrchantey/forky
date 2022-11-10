use crate::{LedsInterface, WasmEngine, WasmInstance, WasmInstanceBuilder};
use std::sync::{Arc, Mutex};
use std::{cell::RefCell, rc::Rc, time::SystemTime};
use wasmi::{Caller, TypedFunc};

type Store = [u8; 16];
pub type SketchBuilder = WasmInstanceBuilder<Store>;

pub struct SketchInstance {
	run: TypedFunc<(), ()>,
	_millis: TypedFunc<(), u64>,
	instance: WasmInstance<Store>,
}

type A = ();

impl SketchInstance {
	pub fn init(engine: &mut WasmEngine) -> SketchBuilder {
		let mut store: Store = [0; 16];
		engine.instantiate(store)
	}

	pub fn build(
		engine: &mut WasmEngine,
		mut builder: SketchBuilder,
		stream: &Vec<u8>,
	) -> SketchInstance {
		let mut instance = builder.build(engine, &stream[..]);
		let run = instance.get_export::<(), ()>("run");
		let _millis = instance.get_export::<(), u64>("_millis");
		SketchInstance {
			run,
			_millis,
			instance,
		}
	}

	pub fn append_millis(builder: &mut SketchBuilder) {
		let start_time = SystemTime::now();
		builder.add_import_0("host", "millis", move |caller| {
			let now = SystemTime::now();
			let elapsed = now.duration_since(start_time).unwrap();
			elapsed.as_millis() as u64
		});
	}


	pub fn run(&mut self) {
		self.run.call(&mut self.instance.store, ()).unwrap();
	}
	pub fn _millis(&mut self) -> u64 {
		self._millis.call(&mut self.instance.store, ()).unwrap()
	}
}
