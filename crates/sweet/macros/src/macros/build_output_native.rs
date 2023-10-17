use super::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn parse_test_case_native(
	func: &TokenStream,
	flags: &TestCaseFlags,
) -> TokenStream {
	let config = flags.to_config();
	let func = parse_func_native(func, flags);
	// let id = flags.id;
	let name = flags.name.clone();

	quote!(
		#[cfg(not(target_arch = "wasm32"))]
		inventory::submit!(sweet::native::TestCaseNative {
			name: #name,
			func: #func,
			file: file!(),
			config: #config
		});
	)
}

fn parse_func_native(func: &TokenStream, flags: &TestCaseFlags) -> TokenStream {
	let contains_await = contains_await(func);
	if contains_await {
		if flags.non_send {
			quote!(sweet::native::TestCaseNativeFunc::Series(||{
				Box::pin(async {
					#func
					Ok(())
				})
			}))
		} else {
			quote!(sweet::native::TestCaseNativeFunc::Parallel(||{
				Box::pin(async {
					#func
					Ok(())
				})
			}))
		}
	} else {
		quote!(sweet::native::TestCaseNativeFunc::Sync(||{
			#func
			Ok(())
		}))
	}

	// quote!(
	// 	// #[cfg(not(target_arch = "wasm32"))]
	// 	|| {
	// 		Box::pin(async {
	// 			#func
	// 			Ok(())
	// 		})
	// 	}
	// )
}
