use proc_macro2::Ident;
use proc_macro2::Literal;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use std::sync::atomic::*;

pub fn to_inventory_wrap_func(
	name: Literal,
	func: TokenStream,
	config: TokenStream,
) -> TokenStream {
	let func = quote!(||->anyhow::Result<()>{
		#func
		Ok(())
	});
	to_inventory(name, func, config)
}

static CNT: AtomicUsize = AtomicUsize::new(0);

pub fn to_inventory(
	name: Literal,
	func: TokenStream,
	config: TokenStream,
) -> TokenStream {
	let wasm_export_name = format!("_sweet_{}", CNT.fetch_add(1, Ordering::SeqCst));
	let wasm_export_name = Ident::new(&wasm_export_name, Span::call_site());

	quote!(
		#[cfg(not(target_arch = "wasm32"))]
		inventory::submit!(sweet::TestCaseDesc {
			name: #name,
			func: #func,
			file: file!(),
			config: #config
		});
		#[cfg(target_arch = "wasm32")]
		use wasm_bindgen::prelude::*;
		#[cfg(target_arch = "wasm32")]
		#[wasm_bindgen]
		pub fn #wasm_export_name() -> JsValue {
			use js_sys::*;
			use wasm_bindgen::prelude::*;

			let obj = Object::new();
			let func: Closure<dyn Fn() -> JsValue> = Closure::new(|| {
				match #func(){
					Ok(_)=> JsValue::NULL,
					Err(e)=> e.to_string().into()
				}
			});
			let func = func.into_js_value();
			Reflect::set(&obj, &"name".into(), &#name.into()).unwrap();
			Reflect::set(&obj, &"file".into(), &file!().into()).unwrap();
			Reflect::set(&obj, &"func".into(), &func).unwrap();
			Reflect::set(&obj, &"config".into(), &"foo".into()).unwrap();
			obj.into()
		}
	)
}
