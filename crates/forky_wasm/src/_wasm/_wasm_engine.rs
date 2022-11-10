use std::{thread, time};
use wasmi::*;

use crate::{WasmInstance, WasmInstanceBuilder};

pub fn run_wasm() {}


pub struct WasmEngine {
	pub engine: Engine,
}

impl WasmEngine {
	pub fn new() -> WasmEngine {
		WasmEngine {
			engine: Engine::default(),
		}
	}

	pub fn instantiate<T>(
		&mut self,
		initial_state: T,
	) -> WasmInstanceBuilder<T> {
		WasmInstanceBuilder::new(self, initial_state)
	}
}
