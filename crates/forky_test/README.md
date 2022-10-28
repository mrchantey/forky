# Forky Test

> *At this stage Forky is really just a proof of concept, do not use seriously!*

Basically a [jest](https://jestjs.io/) clone, the `forky_test` crate will set you up with a beautiful test harness and intuitive matchers that are easy on the eyes.
## Features

### Nested Tests

Create tests in nested directories without touching your `Cargo.toml`.

### Pretty success messages

![success](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/success.png)

### In progress indication

![progress](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/progress.png)

### Failure context

![failure](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/failure.png)

## Getting started
Add the following files:

`Cargo.toml`
```toml
[[test]]
name = "forky"
path = "tests/main.rs"
harness = false
```

`tests/main.rs`
```rust
use forky_test::*;
mod my_test;

fn main() -> Result<(), MatcherError> {
	TestRunner::run()
}
```

`tests/my_test.rs`
```rust
use forky_test::*;

describe!("my test", |s| {
	s.it("works", || {
		expect(true).to_be_false()?;

		Ok(())
	});
});
```

## Commands

- Run 
   - `cargo test --test forky`
- With watch
   - `cargo watch -q -x 'test --test forky -- -w'`
   - Clears terminal on each run
   - Returns an exit code zero (cleaner)
- Specify filename
   - `cargo test --test forky -- my_test`

## Reference
- [speculate](https://github.com/utkarshkukreti/speculate.rs)
	- Nicer syntax