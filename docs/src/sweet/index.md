# Sweet

Sweet is a full-stack test framework for Rust. Use a single framework for some or all of the supported test types:

| Type                   | Description                                        |
| ---------------------- | -------------------------------------------------- |
| Native - Unit          | Results-not-panic workflow                         |
| Native - E2E           | Full support for webdriver, fantoccini etc         |
| In-browser - Component | Test indivudual web components, framework agnostic |
| In-browser - E2E       | Run e2e tests on actual elements via iframes       |

The in-browser tests are architecturally similar to Cypress [Component][1] and [e2e][2] tests. The native tests may be be compared to the likes of [Jest][3] or [Playwright][4].

[1]: https://docs.cypress.io/guides/core-concepts/testing-types#What-is-Component-Testing
[2]: https://docs.cypress.io/guides/core-concepts/testing-types#What-is-E2E-Testing
[3]: https://jestjs.io/
[4]: https://playwright.dev/

## Features

- 🔥 Parallel
- 🕙 Async
- 🕸️ Native & In-Browser
- 🌍 E2E Tests
- ☮️ Intuitive matchers
- 🌈 Pretty output

## Usage

```rs
#[sweet_test]
fn true_is_true() -> Result<()> {
  expect(true).to_be_true()
}
```

## Very Quick Start

```
git clone https://github.com/mrchantey/sweet-demo.git
cd sweet-demo
cargo run --example sweet
```

## Getting Started

Check out the [quickstart page](./native/index.md) or have a browse of the [tests written for sweet](https://github.com/mrchantey/forky/tree/main/crates/sweet/test)

## Overview

Sweet has four main components:

- [`sweet!` defines a test suite](./macros.md)
- [`#[sweet_test]` defines a test](./native/index.md)
- [`expect()` returns a matcher](./matchers.md)
- [`visit()` returns an iframe (e2e)](./web/end-to-end.md)