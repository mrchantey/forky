#![no_std]
#![no_main]
// #![feature(generic_const_exprs)]
// #[allow(unused_imports, unused_variables)]
//general
pub use esp32c3_hal::prelude::*;
pub use esp_backtrace as _;
pub use riscv_rt::entry;
//crate
mod _core;
pub use _core::*;
// mod _utility;
// pub use _utility::*;
