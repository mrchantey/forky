use crate::*;
use anyhow::Result;

pub fn build_simple() -> Result<WasmInstance<u32>> {
	let mut engine = WasmEngine::new();
	let buf = include_wasm!("../../", "wasm_simple");
	let mut builder = engine.instantiate(0);
	builder.add_import_1("host", "howdy", |mut caller, param: i32| {
		let data = caller.host_data_mut();
		*data = *data + param as u32;
	});
	let instance = builder.build(&mut engine, &buf[..]);
	Ok(instance)
}
