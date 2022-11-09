use crate::WasmInstance;
use wasmi::*;

type HostState = (u32);

pub struct WasmInstanceBuilder<'a> {
	// instance: Instance,
	// module: Module,
	engine: &'a mut Engine,
	pub store: Store<HostState>,
	pub linker: Linker<HostState>,
}

impl<'a> WasmInstanceBuilder<'a> {
	pub fn new(engine: &'a mut Engine) -> WasmInstanceBuilder {
		let store = Store::new(engine, 42);
		let linker = <Linker<HostState>>::new();
		WasmInstanceBuilder {
			engine,
			store,
			linker, 
		}
	}

	pub fn add_import<T, P1, P, R>(
		mut self,
		module_name: &str,
		func_name: &str,
		func: T,
	) -> Self
	where
		T: Fn(Caller<u32>, P1) -> () + IntoFunc<u32, P, R>,
	{
		self.linker
			.define(module_name, func_name, Func::wrap(&mut self.store, func))
			.unwrap();
		self
	}

	pub fn build(mut self, stream: impl Read) -> WasmInstance<HostState> {
		let module = Module::new(self.engine, stream).unwrap();
		let instance = self
			.linker
			.instantiate(&mut self.store, &module)
			.unwrap()
			.start(&mut self.store)
			.unwrap();
		WasmInstance {
			store: self.store,
			instance,
		}
	}
}
