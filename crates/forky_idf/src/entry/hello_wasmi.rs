// use anyhow::{anyhow, Result};
use esp_idf_sys as _;
use std::{thread, time};
use wasmi::*;
fn main() {
	std::thread::sleep(time::Duration::from_secs(5));
	// First step is to create the Wasm execution engine with some config.
	// In this example we are using the default configuration.
	let engine = Engine::default();
	// let wat = r#"
	//       (module
	//           (import "host" "hello" (func $host_hello (param i32)))
	//           (func (export "hello")
	//               (call $host_hello (i32.const 3))
	//           )
	//       )
	//   "#;
	// Wasmi does not yet support parsing `.wat` so we have to convert
	// out `.wat` into `.wasm` before we compile and validate it.
	// let wasm = wat::parse_str(&wat).unwrap();
	let wasm = [
		0, 97, 115, 109, 1, 0, 0, 0, 1, 133, 128, 128, 128, 0, 1, 96, 0, 1,
		127, 3, 130, 128, 128, 128, 0, 1, 0, 4, 132, 128, 128, 128, 0, 1, 112,
		0, 0, 5, 131, 128, 128, 128, 0, 1, 0, 1, 6, 129, 128, 128, 128, 0, 0,
		7, 145, 128, 128, 128, 0, 2, 6, 109, 101, 109, 111, 114, 121, 2, 0, 4,
		109, 97, 105, 110, 0, 0, 10, 138, 128, 128, 128, 0, 1, 132, 128, 128,
		128, 0, 0, 65, 42, 11,
	];
	let module = Module::new(&engine, &mut &wasm[..]).unwrap();

	// All Wasm objects operate within the context of a `Store`.
	// Each `Store` has a type parameter to store host-specific data,
	// which in this case we are using `42` for.
	type HostState = u32;
	let mut store = Store::new(&engine, 42);
	let host_hello =
		Func::wrap(&mut store, |caller: Caller<'_, HostState>, param: i32| {
			println!("Got {param} from WebAssembly");
			println!("My host state is: {}", caller.host_data());
		});

	// In order to create Wasm module instances and link their imports
	// and exports we require a `Linker`.
	let mut linker = <Linker<HostState>>::new();
	// Instantiation of a Wasm module requires defining its imports and then
	// afterwards we can fetch exports by name, as well as asserting the
	// type signature of the function with `get_typed_func`.
	//
	// Also before using an instance created this way we need to start it.
	linker.define("host", "hello", host_hello).unwrap();
	let instance = linker
		.instantiate(&mut store, &module)
		.unwrap()
		.start(&mut store)
		.unwrap();
	let hello = instance
		.get_export(&store, "hello")
		.and_then(Extern::into_func)
		.ok_or_else(|| panic!("could not find function \"hello\""))
		.unwrap()
		.typed::<(), ()>(&mut store)
		.unwrap();

	// And finally we can call the wasm!
	hello.call(&mut store, ()).unwrap();
}
