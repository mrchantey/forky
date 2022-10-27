set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

crates := 'forky forky_core forky_cli forky_test forky_play'


default:
	just --list

test crate:
	RUST_BACKTRACE=full cargo watch -q -x 'test -p {{crate}} --test forky -- -w'

all command:
	for file in {{crates}}; do \
		just {{command}} $file; \
	done

mod: 
	cargo watch -q --ignore '**/mod.rs' -x 'run -p forky_cli'

clean crate:
	cargo clean -p {{crate}}

js:
	#!/usr/bin/env node
	console.log('Greetings from JavaScript!')
@list-crates:
	printf "my crates:\n\n"
	for file in {{crates}}; do \
		echo $file; \
	done

publish crate:
	cargo publish -p {{crate}} --allow-dirty

@log argument:
	echo {{argument}}
