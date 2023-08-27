use super::*;
use proc_macro2::Group;
use proc_macro2::Ident;
// use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

pub fn parse_test_case_wasm(
	func: &Group,
	flags: &TestCaseFlags,
) -> TokenStream {
	let config = flags.to_config();
	let func = parse_func_wasm(func);
	let id = flags.id;
	let name = flags.name.clone();
	let wasm_export_name = format!("_sweet_{id}");
	let wasm_export_name = Ident::new(&wasm_export_name, name.span());

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

fn parse_func_wasm(func: &Group) -> TokenStream {
	quote! {
		#[cfg(target_arch = "wasm32")]
		{|| -> Promise {
			use core::result::Result::{Ok,Err};
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
	}
}
