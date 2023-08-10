# Sweet

Sweet is an **DevEx-First** test framework for rust. Whether you're running native, component or e2e tests, you're gonna have a sweet dev experience.

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
		expect("foo").not().to_contain("bar")?;
  }
}
```

Sweet only exposes three functions:

- [`sweet!` defines a test suite](./macros.md)
- [`expect()` returns a matcher](./matchers.md)
- [`visit()` returns an iframe (e2e)](./web/end-to-end.md)