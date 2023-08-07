# Workspaces

Workspaces are fully supported by Sweet, here's a few notes:


## Duplicate Binaries

As per [this PR](https://github.com/rust-lang/cargo/pull/6308) crates that contain identical example names is not supported. For this reason it is recommended to update your `Cargo.toml`.

```diff
[[example]]
- name = "sweet"
+ name = "sweet_crate_a"
path = "test/sweet.rs"
```

Running `cargo run -p crate_a --example sweet_crate_a` is a bit of a mouthfull, I solve this with [just](https://github.com/casey/just):
```sh
#justfile
test crate *args:
	RUST_BACKTRACE={{backtrace}} cargo run -p {{crate}} --example sweet_{{crate}} -- {{args}}
```
Now you can run:
`just test crate_a`

