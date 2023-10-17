# sweet

The Sweet CLI is a tool for building, serving & running tests in-browser. It can:

- Build the tests with `cargo build` & `wasm bindgen`
- Serve the tests on a dev server with live reload
- Run the tests using `chromedriver`

## Usage:
```sh
# run - requires chromedriver
forky sweet --example my_test
# interactive
forky sweet --example my_test --interactive
# workspaces - expects an example called sweet_my_crate
forky sweet --example my_test -p my_crate
# help
forky sweet --example my_test --help
```

## Requirements

- [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/reference/cli.html)
	- `cargo install -f wasm-bindgen-cli`
- [chromedriver](https://chromedriver.chromium.org/downloads)
	- Not required for interactive mode
	- ```sh	
		# windows
		choco install chromedriver
		# mac
		brew install --cask chromedriver
		# linux
		sudo apt install chromium-chromedriver
		```
	- If your chrome version gets updated you will need to update chromedriver too:
	- `choco upgrade chromedriver` etc

## Help

```
forky sweet --help

Arguments:
  [match]...  filter suites by path glob, ie `my_test` or `/e2e/`

Options:
  -p, --package <package>  pass the --package flag to cargo run
      --release            pass the --release flag to cargo run
      --secure             run the dev server with https
      --static <static>    directory for static files (ie .css) used by component tests
  -w, --watch              live reload file changes
      --headed             run the tests with a visible browser window        
  -i, --interactive        just start the server for viewing in your browser 
```