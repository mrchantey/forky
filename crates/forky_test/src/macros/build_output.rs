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
	let func = quote!({
		#[cfg(not(target_arch = "wasm32"))]
		{|| {
			Box::pin(async {
				#func
				Ok(())
			})
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
	let wasm = to_inventory_wasm(&name, &func, &config);
	quote!(
		use sweet::exports::*;
		#[cfg(not(target_arch = "wasm32"))]
		inventory::submit!(sweet::TestCaseNative {
			name: #name,
			func: TestCaseNativeFunc::Series(#func),
			file: file!(),
			config: #config
		});
		#wasm
	)
}


fn to_inventory_wasm(
	name: &Literal,
	func: &TokenStream,
	config: &TokenStream,
) -> TokenStream {
	let id = CNT.fetch_add(1, Ordering::SeqCst);
	let wasm_export_name = format!("_sweet_{id}");
	let wasm_export_name = Ident::new(&wasm_export_name, Span::call_site());

	quote!(
		#[cfg(target_arch = "wasm32")]
			#[wasm_bindgen]
			pub fn #wasm_export_name() -> JsValue {

				let config = serde_json::to_string(&#config).unwrap();
				let func: Closure<dyn Fn() -> Promise> = Closure::new(#func);
				let func = func.into_js_value();

				sweet::build_test_case(
					&#id.into(),
					&#name.into(),
					&file!().into(),
					&func,
					&config.into(),
				)
			}
	)
}