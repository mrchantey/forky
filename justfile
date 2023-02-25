set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

crates := 'forky forky_cli forky_core forky_play forky_test forky_wasm sweet'
# forky_esp
sh := 'C:/tools/cygwin/bin/'
backtrace := '0'
# backtrace := '1'
# backtrace := 'full'

default:
	just --list

@all command:
	for file in {{crates}}; do \
		just {{command}} $file; \
	done

run crate example:
	RUST_BACKTRACE={{backtrace}} cargo run -p {{crate}} --example {{example}}

build crate example:
	RUST_BACKTRACE={{backtrace}} cargo build -p {{crate}} --example {{example}}

check crate:
	cargo check -p {{crate}}

clean *args:
	cargo clean -p {{args}}

clean-repo:
	cargo clean
	just all clean
	rm -rf ./target
# rm -rf C:/temp/.embuild
# rm -rf C:/temp/idf
# rm -rf ./.embuild
# rm -rf ./target-esp


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
	RUST_BACKTRACE={{backtrace}} cargo test -p {{crate}} --test sweet -- {{args}}

test-w crate *args:
	RUST_BACKTRACE={{backtrace}} just watch 'cargo test -p {{crate}} --test sweet -- -w {{args}}'

watch command:
	cargo watch -q --ignore '{**/mod.rs,justfile,.gitignore}' --ignore '**.{txt,md,wasm,wat}' --ignore '{output/,out/}' -- {{command}}
# cargo watch -q --ignore '**/mod.rs' --ignore '**/lib.rs' -- {{command}}
#cargo watch -q --ignore '**/mod.rs' -x '{{command}}'


### WASM ###

run-wasm crate example:
	cargo run -p {{crate}} --example {{example}} --target wasm32-unknown-unknown

build-wasm crate example:
	cargo build -p {{crate}} --example {{example}} --release --target wasm32-unknown-unknown
	RUST_BACKTRACE={{backtrace}} wasm-bindgen \
	--out-dir ./html/wasm \
	--out-name bindgen \
	--target web \
	--no-typescript \
	./target/wasm32-unknown-unknown/release/examples/{{example}}.wasm

watch-wasm *args:
	just watch 'just build-wasm {{args}}'

serve-wasm:
	cd ./html && live-server

### ESP ###

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
	--speed 921600 \
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
