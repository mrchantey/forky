use backtrace::Backtrace;
use forky_core::*;
use sweet::*;

describe!("backtrace", |s| {
	s.it("works", || {
		let ctx = backtracer::file_context();
		expect(ctx.as_str()).to_contain("let ctx = backtracer::file_context();")?;
		Ok(())
	});
});
