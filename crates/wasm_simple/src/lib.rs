#![allow(dead_code)]
#![no_std]

#[link(wasm_import_module = "host")]
extern "C" {
	#[link_name = "hello"]
	// #[no_mangle]
	fn host_hello(val: i32);
}

#[no_mangle]
pub extern "C" fn hello() {
	unsafe {
		let a = 2;
		let b = 10;
		host_hello(a + b);
	}
}
#[no_mangle]
pub extern "C" fn add(a: u64, b: u64) -> u64 { a + b }

#[cfg(not(test))]
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! { unreachable!() }
