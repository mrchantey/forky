use std::cell::RefCell;

use crate::{terminal_leds::TerminalLeds, *};
use forky_core::*;
use forky_wasm::*;
use sweet::*;
use wasmi::*;

sweet! {

	let leds = TerminalLeds::shared();
	let mut instance = SketchInstance::from_default(leds);
	let mut wasm = include_wasm!("../../../","wasm_sketch");
	test "millis" {
		let a = instance._millis();
		forky_core::time::sleep(1);
		let b = instance._millis();
		expect((b - a) as i32).to_be_at_least(1000)?;
	}

	test "leds"{
		instance.run();
	}

	test "size"{
		expect(wasm.len() < 1000).to_be_true();
	}

	test skip "print"{
		println!("{:?}",wasm);
		println!("\n{} bytes\n",wasm.len());
	}
}
