# sweet

> *At this stage sweet is a proof of concept, do not use seriously!*

Basically a [jest](https://jestjs.io/) clone, the `sweet` crate will set you up with a beautiful test harness and intuitive matchers that are easy on the eyes.

## Quickstart
`cargo.toml`
```toml
[dev-dependencies]
sweet = # current version here

[[test]]
name = "sweet"
path = "test/sweet.rs"
harness = false
```
`test/sweet.rs`
```rust
pub use sweet::*;

sweet! {
	it "works" {
		expect(true).to_be_false()?;
	}
}
```

## Features

### Nested Tests

Sweet is designed to collect and run all tests in one go. All tests exposed in the `sweet.rs` file will be run:


```rust
//test/sub_dir/some_test.rs
sweet!{
	it "works" {
		expect(true).to_be_true();
	}
}
//test/sub_dir/mod.rs
mod some_test
//test/sweet.rs
pub mod sweet::*;
mod sub_dir;
```

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
use sweet::*;
mod my_test;

fn main() -> Result<(), MatcherError> {
	test_runner::run()
}
```

`tests/my_test.rs`
```rust
use sweet::*;

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