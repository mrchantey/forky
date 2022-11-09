use forky_wasm::*;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use sweet::*;
use wasmi::*;


fn read_wasm_bytes(path: &str) -> io::Result<(Vec<u8>)> {
	let f = File::open(format!(
		"../wasm_{}/target/wasm32-unknown-unknown/release/wasm_{}.wasm",
		path, path
	))?;
	let mut reader = BufReader::new(f);
	let mut buffer = Vec::new();
	reader.read_to_end(&mut buffer)?;
	Ok(buffer)
}

sweet! {

	let mut engine = WasmEngine::new();
	let mut buf = read_wasm_bytes("simple").unwrap();
	let mut instance = engine.instantiate().add_import(
		"host", "hello",|mut caller, param:i32| {
			let data = caller.host_data_mut();
			*data = *data + param as u32;
		}
	).build(&buf[..]);

	test "exports" {
		let add = instance.get_export::<(u64,u64),u64>("add");
		let result = add.call(&mut instance.store,(1,2)).unwrap();
		expect(result).to_be(3)?;
	}
	test "imports" {
		let hello = instance.get_export::<(),()>("hello");
		hello.call(&mut instance.store,()).unwrap();
		let state = instance.store.state();
		expect(*state).to_be(54)?;

	}
}
