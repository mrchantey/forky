# Contributing

## Quickstart
- `just build-w bevy_webxr`
- Local testing
	- `just serve`
- On-Device
	- `just serve-https`

## Non rust dependencies

These are all pretty much dev's choice for running commands, servers etc. but this is how I do it.

- [just](https://github.com/casey/just)
	- Command runner
	- Alternatively copy/paste commands from [justfile](./justfile)
- [cygwin](https://www.cygwin.com/)
	- Bash shell for running just on Windows
- [live-server](https://www.npmjs.com/package/live-server)
	- feel free to use any server you wish
- [openssl](https://www.openssl.org/)
	- https cert key generation


## `mod.rs` Files

I know this is a little unconventional, but I auto-generate `mod.rs` files based on directory layout. The default rules are that any file and directory with a `_` prefix is exposed as-is. Otherwise directories are treated as 'namespaces'. The `lib.rs` file is not auto-generated 