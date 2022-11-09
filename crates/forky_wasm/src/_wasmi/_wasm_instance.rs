use wasmi::*;

type HostState = (u32);

pub struct WasmInstance<T> {
	pub instance: Instance,
	pub store: Store<T>,
}

impl<T> WasmInstance<T> {
	pub fn new(store: Store<T>, instance: Instance) -> WasmInstance<T> {
		WasmInstance { store, instance }
	}

	pub fn get_export<P, R>(&mut self,name:&str) -> TypedFunc<P, R>
	where
		P: WasmParams,
		R: WasmResults,
		// F: FnOnce(P) -> R,
	{
		self.instance
			.get_export(&self.store, name)
			.and_then(Extern::into_func)
			.ok_or_else(|| panic!("could not find function {}",name))
			.unwrap()
			.typed::<P, R>(&mut self.store)
			.unwrap()

		// let func = |val| hello.call(&mut self.store, val).unwrap();
		// hello.call(&mut self.store, ()).unwrap();
		// func
	}
}
