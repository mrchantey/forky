#![no_std]
#![no_main]

#[allow(unused_imports)]
pub use esp32c3_hal::prelude::*;
use esp_backtrace as _;
// use nb::block;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
	loop {
	}
}
