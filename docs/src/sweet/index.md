# Sweet

Sweet is an authoring-first testing framework for rust. With an ergonomic native runner and an interactive web runner, you're gonna have a sweet dev experience.

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