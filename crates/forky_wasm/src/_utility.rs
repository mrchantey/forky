// #[macro_export]
// macro_rules! include_wasm {
// 	($value:expr) => {
// 		include_bytes!(
// 			"../$value/target/wasm32-unknown-unknown/release/wasm_sketch.wasm"
// 		);
// 	};
// }
#[macro_export]
macro_rules! include_wasm {
	// () => {
	($prefix:expr,$name:expr) => {
		include_bytes!(concat!($prefix,$name,"/target/wasm32-unknown-unknown/release/",$name,".wasm"))
	};
}

// include_bytes!(
// 	"../$value/target/wasm32-unknown-unknown/release/wasm_sketch.wasm"
// );
// pub fn wasm_simple_bytes() -> u8 {
// 	include_bytes!("../../../wasm_sketch/target/wasm32-unknown-unknown/release/wasm_sketch.wasm");
// }
