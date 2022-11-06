set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

crates := 'forky forky_core forky_cli forky_test forky_play sweet'
sh := 'C:/tools/cygwin/bin/'

default:
	just --list

@all command:
	for file in {{crates}}; do \
		just {{command}} $file; \
	done

build crate:
	cargo build -p crate

build-maze:
	cargo build --release --target wasm32-unknown-unknown	-p forky_play --example maze
	wasm-bindgen --out-dir ./out/ --target web ./target/

check crate:
	cargo check -p {{crate}}

clean crate:
	cargo clean -p {{crate}}

expand crate example:
	just watch 'cargo expand -p {{crate}} --example {{example}}'

example crate example *args:
	just watch 'cargo run -p {{crate}} --example {{example}} {{args}}'

@log argument:
	echo {{argument}}

mod: 
	just watch 'cargo run -p forky_cli'

publish crate:
	cargo publish -p {{crate}} --allow-dirty
# publishing all will not work because of equal dependency race

publish-all:
	cargo set-version --bump patch
	just publish forky || true
	{{sh}}sleep 5
	just publish forky_core || true
	{{sh}}sleep 5
	just publish forky_test || true
	{{sh}}sleep 5
	just publish sweet || true
# just publish forky_cli
# just publish forky_play

start crate: 
	./target/debug/{{crate}}.exe

test crate *args:
	RUST_BACKTRACE=1 cargo test -p {{crate}} --test sweet -- {{args}}

test-w crate *args:
	RUST_BACKTRACE=1 just watch 'cargo test -p {{crate}} --test sweet -- -w {{args}}'

watch command:
	cargo watch -q --ignore '{**/mod.rs,justfile,.gitignore}' --ignore '**.{txt,md}' --ignore 'output' -- {{command}}
# cargo watch -q --ignore '**/mod.rs' --ignore '**/lib.rs' -- {{command}}
#cargo watch -q --ignore '**/mod.rs' -x '{{command}}'