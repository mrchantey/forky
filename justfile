set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

crates := 'forky forky_core forky_cli forky_test forky_play sweet'

default:
	just --list

@all command:
	for file in {{crates}}; do \
		just {{command}} $file; \
	done

build crate:
	cargo build -p crate

clean crate:
	cargo clean -p {{crate}}

expand crate example:
	just watch 'cargo expand -p {{crate}} --example {{example}}'

example crate example:
	just watch 'cargo run -p {{crate}} --example {{example}}'

@log argument:
	echo {{argument}}

mod: 
	just watch 'cargo run -p forky_cli'

publish crate:
	cargo publish -p {{crate}} --allow-dirty
# publishing all will not work because of equal dependency race

publish-all:
	just publish forky
	just publish forky_core
	just publish forky_cli
	just publish forky_test
	just publish sweet
# just publish forky_play

start crate: 
	./target/debug/{{crate}}.exe

test crate:
	cargo test -p {{crate}} --test sweet

test-w crate:
	just watch 'cargo test -p {{crate}} --test sweet -- -w'

watch command:
	cargo watch -q --ignore '**/mod.rs' -- {{command}}
# cargo watch -q --ignore '**/mod.rs' --ignore '**/lib.rs' -- {{command}}
#cargo watch -q --ignore '**/mod.rs' -x '{{command}}'
