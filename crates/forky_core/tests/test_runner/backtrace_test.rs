use backtrace::Backtrace;
use forky_core::testing::*;
use forky_core::*;

describe!("backtrace", |s| {
	s.it("works", || {
		let ctx = Backtracer::file_context();
		expect(ctx.contains("let ctx = Backtracer::file_context();")).to_be_true()?;
		// log!(ctx);
		Ok(())
	});

	s.skip().test("fails", || {
		expect(true).to_be(false)?;
		Ok(())
	});
});
