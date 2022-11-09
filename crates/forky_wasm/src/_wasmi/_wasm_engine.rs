use std::{thread, time};
use wasmi::*;

use crate::{WasmInstance, WasmInstanceBuilder};

pub fn run_wasm() {}


pub struct WasmEngine {
	engine: Engine,
}

impl WasmEngine {
	pub fn new() -> WasmEngine {
		WasmEngine {
			engine: Engine::default(),
		}
	}

	pub fn instantiate(&mut self) -> WasmInstanceBuilder {
		WasmInstanceBuilder::new(&mut self.engine)
	}
}
