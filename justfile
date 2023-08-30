set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

crates := 'forky forky_cli forky_core forky_play forky_test sweet'
# testable := 'forky_core forky_cli forky_fs forky_play sweet'
testable := 'sweet forky_play forky_fs forky_cli forky_core'
# features := '--features forky_play/shader_debug_internal'
features := ''
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

run crate example *args:
	RUST_BACKTRACE={{backtrace}} cargo run -p {{crate}} --example {{example}} {{features}} {{args}}

build-why crate example *args:
	RUST_LOG=cargo::ops::cargo_rustc::fingerprint=info cargo build -p {{crate}} --example {{example}} {{args}}

fix crate *args:
	cargo fix --allow-dirty --lib -p {{crate}} {{args}}
fix-all *args:
	cargo fix --allow-dirty --workspace {{args}}

cli *args:
	cargo run -p forky_cli -- {{args}}

install-cli *args:
	cargo install --path ./crates/forky_cli {{args}}

run-w *args:
	just watch just run {{args}}

build crate *args:
	RUST_BACKTRACE={{backtrace}} cargo build -p {{crate}} {{args}}

check crate *args:
	RUST_BACKTRACE={{backtrace}} cargo check -p {{crate}} {{args}}

clean crate *args:
	RUST_BACKTRACE={{backtrace}} cargo clean -p {{crate}} {{args}}

clean-repo:
	cargo clean
	rm -rf ./target
	rm -rf ./Cargo.lock
	just all clean
#just test-all
# rm -rf C:/temp/.embuild
# rm -rf C:/temp/idf
# rm -rf ./.embuild
# rm -rf ./target-esp


# required: cargo binstall cargo-expand
expand crate example *args:
	just watch 'cargo expand -p {{crate}} --example {{example}} {{args}}'
expand-wasm crate example *args:
	just expand {{crate}} {{example}} --target wasm32-unknown-unknown {{args}}

example crate example *args:
	just watch 'cargo run -p {{crate}} --example {{example}} {{args}}'

@log argument:
	echo {{argument}}

patch:
	cargo set-version --bump patch

publish crate *args:
	cargo publish -p {{crate}} --allow-dirty --no-verify {{args}}
	sleep 2

publish-all:
	just publish forky || true
	just publish forky_core || true
	just publish forky_fs || true
	just publish forky_web || true
	just publish forky_test || true
	just publish sweet || true
	just publish forky_cli || true
	just publish forky_ai || true
	just publish forky_play || true

start crate: 
	./target/debug/{{crate}}.exe

test-all *args:
	for file in {{testable}}; do \
		just test $file {{args}} --parallel; \
	done

test crate *args:
	RUST_BACKTRACE={{backtrace}} cargo run -p {{crate}} --example test_{{crate}} -- {{args}}

test-w crate *args:
	just watch just test {{crate}} -w {{args}}

docs:
	cd docs && mdbook serve

watch *command:
	forky watch \
	-w '**/*.rs' \
	-i '{.git,target,html}/**' \
	-i '**/mod.rs' \
	-i '**/*_g.rs' \
	-- {{command}}
### PLAY ###

vis:
	just run forky_play bevy_graph
	just dot-to-svg target/render_graph.dot
dot-to-svg target:
	dot -Tsvg -O {{target}}

bevy-deps:
	cargo search bevy
	cargo search bevy-inspector-egui 
	cargo search bevy_mod_debugdump 
	cargo search bevy_rapier3d 
	cargo search bevy_easings


### WASM ###

run-wasm crate example:
	cargo run -p {{crate}} --example {{example}} --target wasm32-unknown-unknown

build-wasm crate example *args:
	echo building
	just copy-wasm-assets
	cargo build -p {{crate}} --example {{example}} --target wasm32-unknown-unknown {{args}}
	RUST_BACKTRACE={{backtrace}} wasm-bindgen \
	--out-dir ./html/wasm \
	--out-name bindgen \
	--target web \
	./target/wasm32-unknown-unknown/debug/examples/{{example}}.wasm
# --no-typescript \

watch-wasm *args:
	just watch 'just build-wasm {{args}}'
# just watch 'just copy-wasm-assets'

serve-wasm *args:
	cd ./html && live-server --host=0.0.0.0 --watch=wasm/bindgen_bg.wasm,index.html,style.css {{args}}

serve-https *args:
	just serve-wasm --https=https.config.js {{args}}

copy-wasm-assets:
	rm -rf ./html/assets
	cp -r ./crates/forky_play/assets ./html/assets

ssl:
	openssl genrsa -out target/client-key.pem 2048
	openssl req -new -key target/client-key.pem -subj "/CN=$cn\/emailAddress=admin@$cn/C=US/ST=Ohio/L=Columbus/O=Widgets Inc/OU=Some Unit" -out target/client.csr
	openssl x509 -req -in target/client.csr -signkey target/client-key.pem -out target/client-cert.pem

style:
	cargo run -p forky_cli style all

watch-css crate *args:
	forky watch \
	just build-css {{crate}} {{args}} \
	-w '**/*.css' \
	-i '{.git,target,html}/**' \

@build-css crate *args:
	just lightning ./crates/{{crate}}/src/index.css ./html/style.css {{args}}

lightning in out *args:
	lightningcss {{in}} --bundle -m -o {{out}} {{args}}

test-all-wasm:
	just cli sweet -p sweet --example test_sweet_wasm
	just cli sweet -p forky_web --example test_forky_web_wasm

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
