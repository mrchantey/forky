# Contributing

If this project is of interest to you feel free to have a dig around! 

Everything is very early days so there are no doubt lots of bugs and missing features. If you find something that could be improved please open an issue or PR.

## Getting Started

Most of this is for my own reference, but you may find it useful:

1. Install Rust
	- [Installer](https://www.rust-lang.org/tools/install)
	- Use powershell and be sure to carefully follow steps for build tools
2. Install Depedencies
	- ```sh
			choco install just
			choco install cygwin
			# check
			just all check
			# tools
			cargo install cargo-watch
			rustup toolchain install nightly
			rustup component add rustfmt --toolchain nightly
			cargo +nightly fmt
			rustup default nightly
			# test - compilation will take several minutes
			just all test
		```

## Cygwin

Justfiles require cygwin to work on windows.
1. install cygwin
2. add to path: `C:\tools\cygwin\bin`

## Wasm

- follow the [bevy guide](https://bevy-cheatbook.github.io/platforms/wasm.html)
- setup
	```sh
	rustup target install wasm32-unknown-unknown
	cargo install wasm-server-runner
	cargo install -f wasm-bindgen-cli
	#.cargo/config.toml
	[target.wasm32-unknown-unknown]
	runner = "wasm-server-runner"
	```
- run
	```sh
	#run
	cargo run -p forky_play --example maze --target wasm32-unknown-unknown
	#compile
	cargo build -p forky_play --example maze --release --target wasm32-unknown-unknown
	#build bindings
	wasm-bindgen --out-dir ./html/maze --target web ./target/wasm32-unknown-unknown/release/examples/maze.wasm
	cd html && live-server
	```