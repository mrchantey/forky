use anyhow::Error;
use anyhow::Result;
use sweet::*;


/*
Backtrace depth testing is hard. There are normaly three function layers of abstraction:
1. declarative 	ie to_be
2. assertion	 	ie assert_equal
3. error 				ie to_error
*/

fn declarative_level(err: Error) -> Result<()> {
	expect(err.to_string().as_str()).to_contain("backtrace_test.rs")
}

fn assertion_level(err: Error) -> Result<()> {
	expect(err.to_string().as_str()).not().to_contain(".rs")
}

fn error_level(err: Error) -> Result<()> {
	expect(err.to_string().as_str()).not().to_contain(".rs")
	// expect(err.to_string().as_str()).to_contain("catch_unwind.rs")
}

sweet! {

	test "file context" {
		let ctx = file_context();
		expect(ctx.as_str()).to_contain("let ctx = backtracer::file_context();")?;
		// expect(false).to_be_true()?;
	}

	test "level 1: declarative"{
		let result = expect(true).to_be(false);
		declarative_level(result.unwrap_err())?;
	}

	test "level 2: assertion"{
		let result = expect(true).assert_equal(false);
		assertion_level(result.unwrap_err())?;
		let result = expect(true).assert_correct(false,&false);
		assertion_level(result.unwrap_err())?;
		let result = expect(true).assert_correct_with_received(false,&false,&true);
		assertion_level(result.unwrap_err())?;
	}

	test "level 3: error"{
		let err = expect(true).to_error(&false);
		error_level(err)?;
		let err = expect(true).to_error_with_received(&false,&true);
		error_level(err)?;
		let err = expect(true).to_error_with_backtrace(&false,0);
		error_level(err)?;

		// dont think this one is effective
		// let err = expect(true).to_error_with_received_and_backtrace(&false,&true,0);
		// assertion_level(err)?;
	}
}
