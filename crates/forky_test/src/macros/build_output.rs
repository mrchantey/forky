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
	let func = quote!({
		#[cfg(not(target_arch = "wasm32"))]
	{|| {
			use sweet::exports::FutureExt;
			async fn func_async ()->sweet::exports::Result<()>{
				#func
				Ok(())
			};
			async fn panic_caught()->sweet::exports::Result<()>{
				let result = func_async().catch_unwind();
				sweet::unwrap_panic_catch(result).await
			}
			// let panic_res = func_async.catch_unwind();
			// Box::pin(func_async)
			Box::pin(panic_caught())
			// Box::pin(func_async.catch_unwind())
			// sweet::exports::block_on(func_async())
		}}
		#[cfg(target_arch = "wasm32")]
		{|| -> sweet::exports::Promise {
			async fn func_async ()->sweet::exports::Result<()>{
				#func
				Ok(())
			};
			async fn func_to_js()->sweet::exports::Result<sweet::exports::JsValue,sweet::exports::JsValue>{
				match func_async().await{
					Ok(_)=> Ok(sweet::exports::JsValue::NULL),
					Err(e)=> Err(e.to_string().into())
				}
			}

			sweet::exports::future_to_promise(func_to_js())
		}
	}
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
		sweet::exports::inventory::submit!(sweet::TestCaseNative {
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
			let config = #config.to_i32();

			let func: Closure<dyn Fn() -> Promise> = Closure::new(#func);
			let func = func.into_js_value();

			Reflect::set(&obj, &"name".into(), &#name.into()).unwrap();
			Reflect::set(&obj, &"file".into(), &file!().into()).unwrap();
			Reflect::set(&obj, &"func".into(), &func).unwrap();
			Reflect::set(&obj, &"config".into(), &config.into()).unwrap();
			obj.into()
		}
	)
}
