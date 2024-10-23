use forky_core::prelude::*;
use sweet::*;

#[sweet_test]
fn sorted()-> Result<()> {
	let v = vec![0,2,1].sorted();
	expect(v.len()).to_be(3)?;
	expect(v[0]).to_be(0)?;
	expect(v[1]).to_be(1)?;
	expect(v[2]).to_be(2)?;
	
	Ok(())
}

#[sweet_test]
fn first()-> Result<()> {
	let v = vec![0,2,1];
	let v = v._first();
	expect(*v).to_be(0)?;
	
	Ok(())
}