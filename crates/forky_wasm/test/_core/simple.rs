use crate::*;
use forky_wasm::*;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use sweet::*;
use wasmi::*;


sweet! {

	let mut instance = build_simple().unwrap();

	test "exports" {
		let add = instance.get_export::<(u64,u64),u64>("add");
		let result = add.call(&mut instance.store,(1,2)).unwrap();
		expect(result).to_be(3)?;
	}
	test "imports" {
		let hello = instance.get_export::<(),()>("hello");
		hello.call(&mut instance.store,()).unwrap();
		let state = instance.store.state();
		expect(*state).to_be(12)?;

	}
}
