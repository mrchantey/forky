# Passing Arguments

The native binary has a few cli options, to see them all run:

```sh
cargo run --example sweet --help
```

```
Native runner for your tests.

Usage: sweet_sweet.exe [OPTIONS] [match]...

Arguments:
  [match]...  filter suites by path glob, ie. `*/test1.rs */e2e/*`

Options:
  -w, --watch     clears screen and does not return error
  -p, --parallel  run tests in parallel
```