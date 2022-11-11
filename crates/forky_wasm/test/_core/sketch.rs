use std::cell::RefCell;

use crate::{terminal_leds::TerminalLeds, *};
use forky_core::*;
use forky_wasm::*;
use sweet::*;
use wasmi::*;

sweet! {

	let leds = TerminalLeds::shared();
	let mut instance = SketchInstance::from_default(leds);
	test "millis" {
		let a = instance._millis();
		forky_core::time::sleep(1);
		let b = instance._millis();
		expect((b - a) as i32).to_be_at_least(1000)?;
	}

	test "leds"{
		instance.run();
	}

	test skip "print"{
		let mut wasm = include_wasm!("../../../","wasm_sketch");
		println!("{:?}",wasm);
		println!("\n{} bytes\n",wasm.len());
	}
}
