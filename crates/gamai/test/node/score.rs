use gamai::node::Score;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	expect(Score::Fail).to_be(Score::Fail)?;
	expect(Score::Fail).to_be_less_than(Score::Pass)?;
	expect(Score::Fail).to_be_less_than(Score::Weight(0.))?;
	expect(Score::Weight(0.)).to_be_less_than(Score::Pass)?;
	expect(Score::Weight(0.)).to_be_less_than(Score::Weight(0.1))?;

	Ok(())
}
