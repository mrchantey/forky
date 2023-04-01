set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments
sh := 'C:/tools/cygwin/bin/'

crates := 'forky forky_cli forky_core forky_play forky_test sweet'
# forky_esp
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

fix crate *args:
	cargo fix --lib -p {{crate}} {{args}}

run-w *args:
	just watch 'just run {{args}}'

build crate example:
	RUST_BACKTRACE={{backtrace}} cargo build -p {{crate}} --example {{example}}

check crate:
	cargo check -p {{crate}}

clean *args:
	cargo clean -p {{args}}

clean-repo:
	cargo clean
	rm -rf ./target
	rm -rf ./Cargo.lock
	just all clean
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

#https://crates.io/crates/cargo-watch
watch command:
	cargo watch -q --ignore '{**/mod.rs,justfile,.gitignore}' --ignore '**.{txt,md,wasm,wat}' --ignore 'html*' -- {{command}}

### PLAY ###

vis:
	just run forky_play bevy_graph
	just dot-to-svg target/render_graph.dot
dot-to-svg target:
	dot -Tsvg -O {{target}}

### WASM ###

run-wasm crate example:
	cargo run -p {{crate}} --example {{example}} --target wasm32-unknown-unknown

build-wasm crate example:
	echo building
	just copy-wasm-assets
	cargo build -p {{crate}} --example {{example}} --release --target wasm32-unknown-unknown
	RUST_BACKTRACE={{backtrace}} wasm-bindgen \
	--out-dir ./html/wasm \
	--out-name bindgen \
	--target web \
	./target/wasm32-unknown-unknown/release/examples/{{example}}.wasm
# --no-typescript \

watch-wasm *args:
	just watch 'just build-wasm {{args}}'
# just watch 'just copy-wasm-assets'

serve-wasm *args:
	cd ./html && live-server --watch=wasm/bindgen_bg.wasm,index.html {{args}}

copy-wasm-assets:
	rm -rf ./html/assets
	cp -r ./crates/forky_play/assets ./html/assets

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
