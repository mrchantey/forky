### Wasm

1. `just watch-wasm forky_play rotate_cube`
2. `just ssl`
	-	create a long-life ssl cert, this command only needs to be run once
3. `just serve-https`

### Useful Tools

- `cargo install uuid-generate`
- `uuid-generate` can be used for bevy assets etc

### Shader debugging
- dependent crates: `--features forky_play/shader_debug`
- internal: `--features forky_play/shader_debug_internal`
## Reference
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)


### cargo build optimization

- https://nnethercote.github.io/perf-book/compile-times.html
- https://matklad.github.io/2021/09/04/fast-rust-builds.html
- https://xxchan.me/cs/2023/02/17/optimize-rust-comptime-en.html
- https://endler.dev/2020/rust-compile-times/
`cargo build --timings`