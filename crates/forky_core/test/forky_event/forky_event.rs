use forky_core::ForkyEvent;
use sweet::*;

sweet! {
	it "works" {
		let mut e = ForkyEvent::default();
		let func = |val:&u32|{
			expect(*val).to_be(3).unwrap();
			// println!("{val}");
		};
		e.add_listener(Box::new(func));
		e.trigger(&3);
	}
}
