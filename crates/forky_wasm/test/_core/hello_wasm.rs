use crate::*;
use forky_wasm::*;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use sweet::*;
use wasmi::*;


sweet! {

	let mut engine = WasmEngine::new();
	let mut buf = utility::read_wasm_bytes("simple").unwrap();
	let mut builder = engine.instantiate(0);
	builder.add_import_1(
	"host", "hello",|mut caller, param:i32| {
			let data = caller.host_data_mut();
			*data = *data + param as u32;
		}
	);
	let mut instance = builder.build(&mut engine,&buf[..]);

	test "exports" {
		let add = instance.get_export::<(u64,u64),u64>("add");
		let result = add.call(&mut instance.store,(1,2)).unwrap();
		expect(result).to_be(3)?;
	}
	test "imports" {
		let hello = instance.get_export::<(),()>("hello");
		hello.call(&mut instance.store,()).unwrap();
		let state = instance.store.state();
		expect(*state).to_be(12)?;

	}
}
