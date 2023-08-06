# Sweet

## Features

- 🔥 Parallel
- 🕙 Async
- 🌍 WASM UI tests
- ☮️ Intuitive matchers
- 🌈 Pretty output

## Usage

```rs
sweet! {
  it "works" {
		expect("some string").not().to_start_with("foo")?;
  }
}
```