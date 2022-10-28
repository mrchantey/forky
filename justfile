set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

crates := 'forky forky_core forky_cli forky_test forky_play'


default:
	just --list

test crate:
	RUST_BACKTRACE=full cargo test -p {{crate}} --test forky

test-w crate:
	just watch 'test -p {{crate}} --test forky -- -w'

watch command:
	RUST_BACKTRACE=full cargo watch -q --ignore '**/mod.rs' -x '{{command}}'

all command:
	for file in {{crates}}; do \
		just {{command}} $file; \
	done

mod: 
	just watch 'run -p forky_cli'

clean crate:
	cargo clean -p {{crate}}

# publishing all will not work because of equal dependency race
publish crate:
	cargo publish -p {{crate}} --allow-dirty

@log argument:
	echo {{argument}}
