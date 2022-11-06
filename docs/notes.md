## Misc

Nightly format
```sh
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly
cargo +nightly fmt
```

## Cygwin

Justfiles require cygwin to work on windows.
1. install cygwin
2. add to path: `C:\tools\cygwin\bin`
```

```

## Wasm

- follow the [bevy guide](https://bevy-cheatbook.github.io/platforms/wasm.html)
- setup
	```sh
	rustup target install wasm32-unknown-unknown
	cargo install wasm-server-runner
	#.cargo/config.toml
	[target.wasm32-unknown-unknown]
	runner = "wasm-server-runner"
	```
- run
	```sh
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./out/ --target web ./target/
	```