set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]

run:
	pwd

do-thing: run
	dood

test-core: 
	RUST_BACKTRACE=full cargo watch -q -x 'test -p forky_core --test forky -- -w'

auto-mod: 
	cargo watch -q --ignore '**/mod.rs' -x 'run -p forky_fs'

clean: 
	cargo clean -p forky_core && cargo clean -p forky_fs