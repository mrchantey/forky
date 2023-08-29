# Native Runner

The native runner is an alternative to vanilla rust unit and integration tests. It creates a single binary for all of your tests which speeds up compile times, see [this blog](https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html) for more info.

## Usage

The native runner has a few cli options, run with `--help` to see them all.

```sh
cargo run --example sweet --help
```

```
Arguments:
  [match]...  filter suites by path glob, ie `my_test` or `/e2e/`   

Options:
  -w, --watch     clears screen and does not return error
  -p, --parallel  run tests in parallel
  -s, --silent    don't log results
```
