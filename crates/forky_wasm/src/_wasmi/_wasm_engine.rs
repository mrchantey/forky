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

	pub fn run_compiled(&self, wasm: impl Read) {
		let module = Module::new(&self.engine, wasm).unwrap();
		type HostState = (u32);
		let mut store = Store::new(&self.engine, 42);
		let host_hello = Func::wrap(
			&mut store,
			|caller: Caller<'_, HostState>, param: i32| {
				println!("Got {param} from WebAssembly");
			},
		);
		let mut linker = <Linker<HostState>>::new();
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
		hello.call(&mut store, ()).unwrap();
		let add = instance
			.get_export(&store, "add")
			.and_then(Extern::into_func)
			.ok_or_else(|| panic!("could not find function \"add\""))
			.unwrap()
			.typed::<(u64, u64), u64>(&mut store)
			.unwrap();
		let val = add.call(&mut store, (1, 2)).unwrap();
		println!("1 + 2 = {}", val);
	}

	// pub fn run(&self) {
	// 	// First step is to create the Wasm execution engine with some config.
	// 	// In this example we are using the default configuration.
	// 	// let wat = r#"
	// 	//       (module
	// 	//           (import "host" "hello" (func $host_hello (param i32)))
	// 	//           (func (export "hello")
	// 	//               (call $host_hello (i32.const 3))
	// 	//           )
	// 	//       )
	// 	//   "#;
	// 	// Wasmi does not yet support parsing `.wat` so we have to convert
	// 	// out `.wat` into `.wasm` before we compile and validate it.
	// 	// let wasm = wat::parse_str(&wat).unwrap();

	// 	let wasm = [
	// 		0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02,
	// 		0x60, 0x01, 0x7f, 0x00, 0x60, 0x00, 0x00, 0x02, 0x0e, 0x01, 0x04,
	// 		0x68, 0x6f, 0x73, 0x74, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x00,
	// 		0x00, 0x03, 0x02, 0x01, 0x01, 0x07, 0x09, 0x01, 0x05, 0x68, 0x65,
	// 		0x6c, 0x6c, 0x6f, 0x00, 0x01, 0x0a, 0x08, 0x01, 0x06, 0x00, 0x41,
	// 		0x03, 0x10, 0x00, 0x0b,
	// 	];
	// 	let module = Module::new(&self.engine, &mut &wasm[..]).unwrap();

	// 	// All Wasm objects operate within the context of a `Store`.
	// 	// Each `Store` has a type parameter to store host-specific data,
	// 	// which in this case we are using `42` for.
	// 	type HostState = u32;
	// 	let mut store = Store::new(&self.engine, 42);
	// 	let host_hello = Func::wrap(
	// 		&mut store,
	// 		|caller: Caller<'_, HostState>, param: i32| {
	// 			println!("Got {param} from WebAssembly");
	// 			println!("My host state is: {}", caller.host_data());
	// 		},
	// 	);

	// 	// In order to create Wasm module instances and link their imports
	// 	// and exports we require a `Linker`.
	// 	let mut linker = <Linker<HostState>>::new();
	// 	// Instantiation of a Wasm module requires defining its imports and then
	// 	// afterwards we can fetch exports by name, as well as asserting the
	// 	// type signature of the function with `get_typed_func`.
	// 	//
	// 	// Also before using an instance created this way we need to start it.
	// 	linker.define("host", "hello", host_hello).unwrap();
	// 	let instance = linker
	// 		.instantiate(&mut store, &module)
	// 		.unwrap()
	// 		.start(&mut store)
	// 		.unwrap();
	// 	let hello = instance
	// 		.get_export(&store, "hello")
	// 		.and_then(Extern::into_func)
	// 		.ok_or_else(|| panic!("could not find function \"hello\""))
	// 		.unwrap()
	// 		.typed::<(), ()>(&mut store)
	// 		.unwrap();

	// 	// And finally we can call the wasm!
	// 	hello.call(&mut store, ()).unwrap();
	// }
}
