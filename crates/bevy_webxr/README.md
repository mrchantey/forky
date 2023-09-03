# Bevy WebXR

see the [docs](https://mrchantey.github.io/forky/docs/demos/bevy-webxr.html) for demo and usage.




## internal note

run with this `cargo/config.toml`
```toml
[build]
# target = "x86_64-pc-windows-msvc"
target = "wasm32-unknown-unknown"
rustflags = "-A dead_code -A unused_variables -A unused_imports -A unused_parens"

[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"
rustflags = [
	# "-A dead_code -A unused_variables -A unused_imports -A unused_parens",
 "--cfg","web_sys_unstable_apis",
]
```