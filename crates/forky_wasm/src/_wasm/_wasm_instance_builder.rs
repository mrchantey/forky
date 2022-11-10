use crate::{WasmEngine, WasmInstance};
use wasmi::*;

pub struct WasmInstanceBuilder<T> {
	pub store: Store<T>,
	pub linker: Linker<T>,
}

impl<T> WasmInstanceBuilder<T> {
	pub fn new(
		engine: &mut WasmEngine,
		initial_state: T,
	) -> WasmInstanceBuilder<T> {
		let store = Store::new(&engine.engine, initial_state);
		let linker = <Linker<T>>::new();
		WasmInstanceBuilder { store, linker }
	}

	pub fn build(
		mut self,
		engine: &mut WasmEngine,
		stream: impl Read,
	) -> WasmInstance<T> {
		let module = Module::new(&engine.engine, stream).unwrap();
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
	pub fn add_import<F, P, R>(
		mut self,
		module_name: &str,
		func_name: &str,
		func: F,
	) -> Self
	where
		F: IntoFunc<T, P, R>,
	{
		self.linker
			.define(module_name, func_name, Func::wrap(&mut self.store, func))
			.unwrap();
		self
	}
	pub fn add_import_0<F, P, R>(
		&mut self,
		module_name: &str,
		func_name: &str,
		func: F,
	) -> &Self
	where
		F: Fn(Caller<T>) -> R + IntoFunc<T, P, R>,
	{
		self.linker
			.define(module_name, func_name, Func::wrap(&mut self.store, func))
			.unwrap();
		self
	}
	pub fn add_import_1<F, P1, P, R>(
		&mut self,
		module_name: &str,
		func_name: &str,
		func: F,
	) -> &Self
	where
		F: Fn(Caller<T>, P1) -> R + IntoFunc<T, P, R>,
	{
		self.linker
			.define(module_name, func_name, Func::wrap(&mut self.store, func))
			.unwrap();
		self
	}

	pub fn add_import_2<F, P1, P2, P, R>(
		&mut self,
		module_name: &str,
		func_name: &str,
		func: F,
	) -> &Self
	where
		F: Fn(Caller<T>, P1, P2) -> R + IntoFunc<T, P, R>,
	{
		self.linker
			.define(module_name, func_name, Func::wrap(&mut self.store, func))
			.unwrap();
		self
	}
	pub fn add_import_3<F, P1, P2, P3, P, R>(
		&mut self,
		module_name: &str,
		func_name: &str,
		func: F,
	) -> &Self
	where
		F: Fn(Caller<T>, P1, P2, P3) -> R + IntoFunc<T, P, R>,
	{
		self.linker
			.define(module_name, func_name, Func::wrap(&mut self.store, func))
			.unwrap();
		self
	}
	pub fn add_import_4<F, P1, P2, P3, P4, P, R>(
		&mut self,
		module_name: &str,
		func_name: &str,
		func: F,
	) -> &Self
	where
		F: Fn(Caller<T>, P1, P2, P3, P4) -> R + IntoFunc<T, P, R>,
	{
		self.linker
			.define(module_name, func_name, Func::wrap(&mut self.store, func))
			.unwrap();
		self
	}
}
