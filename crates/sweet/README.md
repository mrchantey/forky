# Sweet

<div align="center">

  <p>
    <strong>Declarative native & web test framework delivering a sweet dev experience.</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/sweet"><img src="https://img.shields.io/crates/v/sweet.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/sweet"><img src="https://img.shields.io/crates/d/sweet.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/sweet"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>

  <h3>
    <a href="https://mrchantey.github.io/forky/docs/sweet">Guide</a>
    <span> | </span>
    <a href="https://docs.rs/sweet">API Docs</a>
    <span> | </span>
    <a href="https://mrchantey.github.io/forky/docs/other/contributing.html">Contributing</a>
  </h3>

  <sub>made with ❤️‍🔥 by mrchantey</a></sub>
</div>

## Usage

```rs
sweet! {
  it "works" {
		expect("some string").not().to_start_with("foo")?;
  }
}
```