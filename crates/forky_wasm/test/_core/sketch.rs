use std::cell::RefCell;

use crate::{terminal_leds::TerminalLeds, *};
use forky_core::*;
use forky_wasm::*;
use sweet::*;
use wasmi::*;

sweet! {

	let leds = TerminalLeds::shared();
	let mut engine = WasmEngine::new();
	let mut wasm = utility::read_wasm_bytes("sketch").unwrap();
	let mut builder = SketchInstance::init(&mut engine);
	SketchInstance::append_millis(&mut builder);
	Led::append_set_rgbw(&mut builder,&leds);
	let mut instance = SketchInstance::build(&mut engine, builder, &wasm);
	test "millis" {
		let a = instance._millis();
		forky_core::time::sleep(1);
		let b = instance._millis();
		expect((b - a) as i32).to_be_at_least(1000)?;
	}

	test "leds"{
		instance.run();

	}


}
