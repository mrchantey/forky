# Sweet

Sweet is a declarative full-stack test framework for Rust with many supported test types:


| Type                   | Analagous To                          | Features                           |
| ---------------------- | ------------------------------------- | ---------------------------------- |
| Native - Vanilla       | [Jest](https://jestjs.io/)            | Declarative matchers               |
| Native - E2E           | [Playwright](https://playwright.dev/) | Uses webdriver, fantoccini etc     |
| In-browser - Component | [Cypress - Component][1]              | Web framework agnostic             |
| In-browser - E2E       | [Cypress - E2E][2]                    | Uses reverse proxy for interaction |

[1]: https://docs.cypress.io/guides/core-concepts/testing-types#What-is-Component-Testing
[2]: https://docs.cypress.io/guides/core-concepts/testing-types#What-is-E2E-Testing

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

Sweet has three main functions:

- [`sweet!` defines a test suite](./macros.md)
- [`expect()` returns a matcher](./matchers.md)
- [`visit()` returns an iframe (e2e)](./web/end-to-end.md)


## Getting Started

Check out the [quickstart page](./native/index.md) or have a browse of the [tests written for sweet](https://github.com/mrchantey/forky/tree/main/crates/sweet/test)