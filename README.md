# Forky Rust Utilities

> *At this stage these crates are really experimental, do not use seriously!*

Each tool is standalone and can be used independently of the others.
- [Forky Test](./crates/forky_test/README.md)
- [Forky CLI](./crates/forky_cli/README.md)


## Development

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

## Contributing

If this project is of interest to you feel free to have a dig around! 

All commands used are in the `justfile`, for example to run a test:
```shX
just test forky_core
```

## Reference
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## Quickstart
[notes](docs/notes.md)