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
			async fn func_async ()->Result<()>{
				#func
				Ok(())
			};
			async fn panic_caught()->Result<()>{
				let result = func_async().catch_unwind();
				sweet::unwrap_panic_catch(result).await
			}
			Box::pin(panic_caught())
		}}
		#[cfg(target_arch = "wasm32")]
		{|| -> Promise {
			async fn func_async ()->Result<()>{
				#func
				Ok(())
			};
			async fn func_to_js()->Result<JsValue,JsValue>{
				match func_async().await{
					Ok(_)=> Ok(JsValue::NULL),
					Err(e)=> Err(e.to_string().into())
				}
			}

			future_to_promise(func_to_js())
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
		use sweet::exports::*;
		#[cfg(not(target_arch = "wasm32"))]
		inventory::submit!(sweet::TestCaseNative {
			name: #name,
			func: #func,
			file: file!(),
			config: #config
		});
		#[cfg(target_arch = "wasm32")]
		#[wasm_bindgen]
		pub fn #wasm_export_name() -> JsValue {

			let config = #config.to_i32();
			let func: Closure<dyn Fn() -> Promise> = Closure::new(#func);
			let func = func.into_js_value();

			sweet::build_test_case(
				&#name.into(),
				&file!().into(),
				&func,
				&config.into(),
			)
		}
	)
}
