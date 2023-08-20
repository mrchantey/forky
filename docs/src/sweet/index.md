# Sweet

Sweet is a full-stack test framework for rust. 

Currently supported test types:

- Native
- Native e2e
	- Use webdriver, fantoccini etc
- In-browser component
	- Runs in isolated iframe
	- Framework agnostic: Yew, Leptos etc
- In-browser e2e
	- Visit page in isolated iframe
	- Uses reverse proxy to interact with page

## Features

- 🔥 Parallel
- 🕙 Async
- 🕸️ Native & Browser
- 🌍 E2E Tests
- ☮️ Intuitive matchers
- 🌈 Pretty output

## Usage

```rs
sweet! {
  it "works" {
		expect(true).to_be_true()?;
  }
}
```

Sweet only exposes three functions:

- [`sweet!` defines a test suite](./macros.md)
- [`expect()` returns a matcher](./matchers.md)
- [`visit()` returns an iframe (e2e)](./web/end-to-end.md)


## Getting Started

Check out the [quickstart page](./native/index.md) or have a browse of the [tests written for sweet](https://github.com/mrchantey/forky/tree/main/crates/sweet/test)