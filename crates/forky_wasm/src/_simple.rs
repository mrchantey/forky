use crate::*;
use anyhow::Result;
use wasmi::*;
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

pub fn build_simple_slowly() -> Result<WasmInstance<u32>> {
	let mut engine = WasmEngine::new();
	let buf = include_wasm!("../../", "wasm_simple");
	let mut builder = engine.instantiate(0);
	// builder.add_import_1("host", "howdy", |mut caller, param: i32| {
	// let data = caller.host_data_mut();
	// *data = *data + param as u32;
	// });

	builder.linker.define(
		"host",
		"howdy",
		Func::wrap(&mut builder.store, |_: Caller<'_, u32>, param: i32| {
			println!("the value is {}", param);
		}),
	)?;

	let mut instance = builder.build(&mut engine, &buf[..]);
	let hello = instance.get_export::<(), ()>("hello");
	hello.call(&mut instance.store, ()).unwrap();

	// let module = Module::new(&engine.engine, &buf[..])?;
	// let _instance = builder
	// 	.linker
	// 	.instantiate(&mut builder.store, &module)?
	// 	.start(&mut self.store)
	// 	.unwrap();
	Ok(instance)
}
