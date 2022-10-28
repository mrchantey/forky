set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

crates := 'forky forky_core forky_cli forky_test forky_play'


default:
	just --list

test crate:
	cargo test -p {{crate}} --test forky

test-w crate:
	just watch 'test -p {{crate}} --test forky -- -w'

watch command:
	cargo watch -q --ignore '**/mod.rs' -x '{{command}}'

watch-cmd command:
	cargo watch -q --ignore '**/mod.rs' -- {{command}}

all command:
	for file in {{crates}}; do \
		just {{command}} $file; \
	done

build crate:
	cargo build -p crate

mod: 
	just watch 'run -p forky_cli'

start crate: 
	./target/debug/{{crate}}.exe

clean crate:
	cargo clean -p {{crate}}

# publishing all will not work because of equal dependency race
publish crate:
	cargo publish -p {{crate}} --allow-dirty

@log argument:
	echo {{argument}}
