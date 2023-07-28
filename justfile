set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

crates := 'forky forky_cli forky_core forky_play forky_test mystic sweet'
testable := 'forky_core forky_cli forky_fs forky_play mystic sweet'
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

fix crate *args:
	cargo fix --allow-dirty --lib -p {{crate}} {{args}}
fix-all *args:
	cargo fix --allow-dirty --workspace {{args}}

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

auto-fs:
	cargo run -p forky_cli auto-fs

mod:
	cargo run -p forky_cli mod

publish crate *args:
	cargo publish -p {{crate}} --allow-dirty {{args}}

publish-all:
	cargo set-version --bump patch
	just publish forky || true
	sleep 5
	just publish forky_core || true
	sleep 5
	just publish forky_test || true
	sleep 5
	just publish forky_fs || true
	sleep 5
	just publish sweet || true
	sleep 5
	just publish forky_cli || true
	sleep 5
	just publish forky_ai || true
	sleep 5
	just publish mystic || true
	sleep 5
	just publish forky_play || true

start crate: 
	./target/debug/{{crate}}.exe


test-all *args:
	for file in {{testable}}; do \
		just test $file {{args}}; \
	done

test crate *args:
	RUST_BACKTRACE={{backtrace}} cargo test -p {{crate}} --test sweet -- {{args}}

test-w crate *args:
	RUST_BACKTRACE={{backtrace}} just watch 'cargo test -p {{crate}} --test sweet -- -w {{args}}'

#https://crates.io/crates/cargo-watch
watch command:
	cargo watch -c -q --ignore '{**/mod.rs,**/*_g.rs,justfile,.gitignore}' --ignore '**.{txt,md,wasm,wat,wgsl,css}' --ignore 'html*' -- {{command}}

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
	cargo watch --ignore '{justfile,.gitignore}' --ignore '**.{rs,txt,md,wasm,wat,wgsl}' --ignore './html/style.css' -- just build-css {{crate}} {{args}}

@build-css crate *args:
	just lightning ./crates/{{crate}}/src/index.css ./html/style.css {{args}}

lightning in out *args:
	lightningcss {{in}} --bundle -m -o {{out}} {{args}}


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
