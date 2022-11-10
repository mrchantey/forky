set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

crates := 'forky forky_core forky_cli forky_test forky_play forky_esp forky_wasm sweet wasm_simple wasm_sketch'
sh := 'C:/tools/cygwin/bin/'

default:
	just --list

@all command:
	for file in {{crates}}; do \
		just {{command}} $file; \
	done

build crate:
	cargo build -p crate

serve-wasm:
	cd ./wasm && live-server

check crate:
	cargo check -p {{crate}}

clean crate:
	cargo clean -p {{crate}}

expand crate example:
	just watch 'cargo expand -p {{crate}} --bin {{example}}'
expand-example crate example:
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

purge:
	rm -rf C:/temp/.embuild
	rm -rf C:/temp/idf
	rm -rf ./.embuild
	rm -rf ./out
	rm -rf ./target-esp
	cargo clean

start crate: 
	./target/debug/{{crate}}.exe

backtrace := '0'

test crate *args:
	RUST_BACKTRACE={{backtrace}} cargo test -p {{crate}} --test sweet -- {{args}}

test-w crate *args:
	RUST_BACKTRACE={{backtrace}} just watch 'cargo test -p {{crate}} --test sweet -- -w {{args}}'

watch command:
	cargo watch -q --ignore '{**/mod.rs,justfile,.gitignore}' --ignore '**.{txt,md,wasm,wat}' --ignore '{output/,out/}' -- {{command}}
# cargo watch -q --ignore '**/mod.rs' --ignore '**/lib.rs' -- {{command}}
#cargo watch -q --ignore '**/mod.rs' -x '{{command}}'

# WASM
# ie just wasm build simple
@wasm command *args:
	just wasm-{{command}} {{args}}

wasm-w command bin *args:
	just watch 'just wasm-{{command}} {{bin}} {{args}}'

wasm-build bin *args:
	cd ./crates/wasm_{{bin}} && cargo build --release --target wasm32-unknown-unknown

wasm-wat bin *args:
	C:/path/wabt/bin/wasm2wat.exe ./target/wasm32-unknown-unknown/release/wasm_{{bin}}.wasm
# wasm-bindgen --out-dir ./wasm/{{example}} --target web ./target/wasm32-unknown-unknown/release/examples/{{example}}.wasm


# ESP

port := 'COM3'
# port := 'COM4'

target-esp := '--target riscv32imc-unknown-none-elf -Zbuild-std=core'

@esp command *args:
	just esp-{{command}} {{args}}
@esp-w command *args:
	just watch 'just esp-{{command}} {{args}}'

esp-build *args:
	cargo build \
	-p forky_esp \
	{{target-esp}} \
	--bin {{args}}

esp-flash *args:
	cargo espflash {{port}} \
	--monitor --release \
	--package forky_esp \
	{{target-esp}} \
	--bin {{args}}

esp-save bin *args:
	cargo espflash save-image \
	--package forky_esp --release \
	{{target-esp}} \
	--bin {{bin}} \
	ESP32-C3 out/esp.image {{args}}

esp-info:
	cargo espflash board-info {{port}}

esp-monitor:
	cargo espflash serial-monitor {{port}}

idf *args:
	cd ./crates/forky_idf; just {{args}}
@idf-w *args:
	just watch 'just idf {{args}}'
