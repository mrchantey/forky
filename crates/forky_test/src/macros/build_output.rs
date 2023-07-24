use proc_macro2::Ident;
use proc_macro2::Literal;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use std::sync::atomic::*;



// fn wrap_async(func: TokenStream) -> TokenStream {
// 	quote!(async move { #func }.await)
// }


pub fn to_inventory_wrap_func(
	name: Literal,
	func: TokenStream,
	config: TokenStream,
) -> TokenStream {
	let func = quote!(|| -> anyhow::Result<()> {
		async fn func_async ()->anyhow::Result<()>{
			#func
			Ok(())
		};
		async_std::task::block_on(func_async())
	});
	to_inventory(name, func, config)
}

static CNT: AtomicUsize = AtomicUsize::new(0);

pub fn to_inventory(
	name: Literal,
	func: TokenStream,
	config: TokenStream,
) -> TokenStream {
	let wasm_export_name =
		format!("_sweet_{}", CNT.fetch_add(1, Ordering::SeqCst));
	let wasm_export_name = Ident::new(&wasm_export_name, Span::call_site());

	quote!(
		#[cfg(not(target_arch = "wasm32"))]
		inventory::submit!(sweet::TestCaseNative {
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
			use wasm_bindgen::prelude::*;
			use js_sys::*;

			let obj = Object::new();
			let func: Closure<dyn Fn() -> JsValue> = Closure::new(|| {
				match #func(){
					Ok(_)=> JsValue::NULL,
					Err(e)=> e.to_string().into()
				}
			});
			let config = #config.to_i32();
			let func = func.into_js_value();
			Reflect::set(&obj, &"name".into(), &#name.into()).unwrap();
			Reflect::set(&obj, &"file".into(), &file!().into()).unwrap();
			Reflect::set(&obj, &"func".into(), &func).unwrap();
			Reflect::set(&obj, &"config".into(), &config.into()).unwrap();
			obj.into()
		}
	)
}
