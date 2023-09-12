use super::*;
use proc_macro2::Group;
use proc_macro2::TokenStream;
use quote::quote;

pub fn parse_test_case_native(
	func: &Group,
	flags: &TestCaseFlags,
) -> TokenStream {
	let config = flags.to_config();
	let func = parse_func_native(func, flags);
	// let id = flags.id;
	let name = flags.name.clone();

	quote!(
		#[cfg(not(target_arch = "wasm32"))]
		inventory::submit!(sweet::TestCaseNative {
			name: #name,
			func: #func,
			file: file!(),
			config: #config
		});
	)
}

fn parse_func_native(func: &Group, flags: &TestCaseFlags) -> TokenStream {
	let contains_await = _contains_await(func.stream());
	if contains_await {
		if flags.non_send {
			quote!(sweet::TestCaseNativeFunc::Series(||{
				Box::pin(async {
					#func
					Ok(())
				})
			}))
		} else {
			quote!(sweet::TestCaseNativeFunc::Parallel(||{
				Box::pin(async {
					#func
					Ok(())
				})
			}))
		}
	} else {
		quote!(sweet::TestCaseNativeFunc::Sync(||{
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
