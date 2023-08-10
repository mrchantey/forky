# sweet

Currently the `sweet` CLI has one function which is to run the interactive web runner.

## Usage:
```sh
forky sweet
```

```
Arguments:
  [specify-suite]...

Options:
  -p, --package <package>  pass the --package flag to cargo run
      --release            pass the --release flag to cargo run
      --secure             run the dev server with https
      --static <static>    directory for static files (ie .css) that should be served  
```

## Features
- Very Hot Reload - The runner features a lightweight dev server that uses `tower-livereload` instead of shutting down and restarting.