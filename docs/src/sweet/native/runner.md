# Native Runner

## Usage

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

## Features

- Single Binary - The default rust intergration test runner creates a seperate binary for each test, which ramps up compile times, see [this blog](https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html) for more info.
- Informative Outputs
	- Long running tests show which suite is hanging
  	![progress](../images/progress.png)