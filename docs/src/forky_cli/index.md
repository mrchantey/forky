# Command Line Interface

### Installation

```
cargo install forky_cli
forky --help

	Usage: forky.exe [COMMAND]
	
	Commands:
	  auto-fs  generate mod and css files
	  watch    execute command on file change
	  serve    serve static files
	  style    Generate types for styles
	  mod      generate mod files for your project
	  sweet    build the wasm sweet runner and start a dev server
```

## Watch

```
Usage: forky.exe watch [OPTIONS] <cmd>...

Arguments:
  <cmd>...  the space seperated command to run, ie forky watch -- echo howdy

Options:
  -w, --watch <watch>    paths to watch
  -i, --ignore <ignore>  paths to ignore
      --once             only run once instead of watching indefinitely
```

## Sweet

```
Usage: forky.exe sweet [OPTIONS] [specify-suite]...

Arguments:
  [specify-suite]...

Options:
  -p, --package <package>  pass the --package flag to cargo run
      --release            pass the --release flag to cargo run
      --secure             run the dev server with https
      --static <static>    directory for static files (ie .css) that should be served  
```