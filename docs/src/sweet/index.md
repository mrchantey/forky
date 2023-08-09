# Sweet

Sweet is an **DevEx-First** test framework for rust. Whether you're running native, component or e2e tests, you're gonna have a sweet dev experience.

## Features

- 🔥 Parallel
- 🕙 Async
- 🕸️ WASM UI tests
- 🌍 E2E Tests
- ☮️ Intuitive matchers
- 🌈 Pretty output

## Usage

```rs
sweet! {
  it "works" {
		expect("bar").not().to_contain("foo")?;
  }
}
```

Sweet only exposes three functions:

- `sweet!` macro
- `expect()`
- `visit()` for e2e tests