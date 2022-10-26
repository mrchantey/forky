# Forky Rust Utilities

Why Forky?

As a web dev I'm used to super slick tooling and need the same in my rust enviroment.


## Tools

Each tool is standalone and can be used independently of the others.

### Forky Test

Basically a jest clone, the forky_test crate will set you up with a beautiful test harness and intuitive matchers that are easy on the eyes.

### Forky Auto Mod

Tired of manually creating and editing `mod.rs` files? The `forky_auto_mod` crate may be for you!
Its current incarnation is zero config and opinionated so you may want to play around with it on an empty project before integrating with existing codebases.

### Forky Core

A bunch of utilities used internally by the others.

## Misc

common commands:
```
cargo watch -q -x 'test -p core'

```


Nightly format

```sh
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly
cargo +nightly fmt
```

Other
```sh
cargo install wasm-bindgen-cli

```
