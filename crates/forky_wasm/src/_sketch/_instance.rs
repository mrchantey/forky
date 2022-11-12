use crate::*;
use std::sync::{Arc, Mutex};
use std::{cell::RefCell, rc::Rc, time::SystemTime};
use wasmi::{Caller, Read, TypedFunc};

// type Store = [u8; 16];
type Store = u32;
pub type SketchBuilder = WasmInstanceBuilder<Store>;

pub struct SketchInstance {
	pub run: TypedFunc<(), ()>,
	pub _millis: TypedFunc<(), u64>,
	pub instance: WasmInstance<Store>,
}


impl SketchInstance {
	pub fn from_default(leds: &SharedLeds) -> SketchInstance {
		let mut engine = WasmEngine::new();
		Self::from_default_with_engine(&mut engine, leds)
	}
	pub fn from_default_with_engine(
		engine: &mut WasmEngine,
		leds: &SharedLeds,
	) -> SketchInstance {
		let stream = include_wasm!("../../../", "wasm_sketch");
		Self::new_with_engine(engine, &stream[..], leds)
	}

	pub fn new(stream: impl Read, leds: &SharedLeds) -> SketchInstance {
		let mut engine = WasmEngine::new();
		Self::new_with_engine(&mut engine, stream, leds)
	}
	pub fn new_with_engine(
		engine: &mut WasmEngine,
		stream: impl Read,
		leds: &SharedLeds,
	) -> SketchInstance {
		let mut builder = SketchInstance::init(engine);
		SketchInstance::append_millis(&mut builder);
		Led::append_imports(&mut builder, &leds);
		SketchInstance::build(engine, builder, stream)
	}


	pub fn init(engine: &mut WasmEngine) -> SketchBuilder {
		let mut store: Store = 69;
		// let mut store: Store = [0; 16];
		engine.instantiate(store)
	}

	pub fn build(
		engine: &mut WasmEngine,
		mut builder: SketchBuilder,
		stream: impl Read,
	) -> SketchInstance {
		let mut instance = builder.build(engine, stream);
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
