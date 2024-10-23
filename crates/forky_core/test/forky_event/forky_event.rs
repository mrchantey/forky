use forky_core::prelude::*;
use sweet::*;

#[sweet_test]
fn works()-> Result<()> {
	let mut e = ForkyEvent::default();
	let func = |val:&u32|{
		expect(*val).to_be(3).unwrap();
		// println!("{val}");
	};
	e.add_listener(Box::new(func));
	e.trigger(&3);
	
	Ok(())
}
