#![allow(dead_code)]
#![no_std]

#[link(wasm_import_module = "host")]
extern "C" {
	// #[link_name = "millis"]
	// #[no_mangle]
	fn millis() -> u64;
}
#[link(wasm_import_module = "led")]
extern "C" {
	fn set_rgbw(r:u32,g:u32,b:u32,w:u32);
}
#[no_mangle]
pub extern "C" fn run() {
	unsafe {
		let _millis = millis();
		set_rgbw(0,0,0,0);
	}
}
#[no_mangle]
pub extern "C" fn _millis() -> u64 { unsafe { millis() } }

#[cfg(not(test))]
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! { unreachable!() }
