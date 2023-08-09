# watch

Analagous to `cargo watch` but allows for watch globs.

usage: 
```sh
forky watch
```

```
Usage: forky.exe watch [OPTIONS] <cmd>...

Arguments:
  <cmd>...  the space seperated command to run, ie forky watch -- echo howdy

Options:
  -w, --watch <watch>    paths to watch
  -i, --ignore <ignore>  paths to ignore
      --once             only run once instead of watching indefinitely
```