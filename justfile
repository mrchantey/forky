set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

crates := 'forky forky_cli forky_core forky_play forky_bevy'
# features := '--features forky_play/shader_debug_internal'
features := ''
# forky_esp
backtrace := '0'
# backtrace := '1'
# backtrace := 'full'

default:
	just --list --unsorted


### common ###

@all command:
	for file in {{crates}}; do \
		just {{command}} $file; \
	done

run crate example *args:
	RUST_BACKTRACE={{backtrace}} cargo run -p {{crate}} --example {{example}} {{args}}

fix *args:
	for file in {{crates}}; do \
			cargo fix --allow-dirty --lib -p $file {{args}}; \
	done

fmt *args:
	for file in {{crates}}; do \
			cargo fmt -p $file {{args}}; \
	done

cli *args:
	cargo run -p forky_cli -- {{args}}

install-cli *args:
	cargo install --path ./crates/forky_cli {{args}}

run-w *args:
	just watch just run {{args}}

build crate example *args:
	RUST_BACKTRACE={{backtrace}} cargo build -p {{crate}} --example {{example}} {{args}}

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

@log argument:
	echo {{argument}}

patch:
	cargo set-version --bump patch

publish crate *args:
	cargo publish -p {{crate}} --allow-dirty --no-verify {{args}}
	sleep 1

publish-all *args:
	just publish forky_core  			{{args}}	| true
	just publish forky_fs  				{{args}}	| true
	just publish forky_net  			{{args}}	| true
	just publish forky_web  			{{args}}	| true
	just publish forky_cli  			{{args}}	| true
	just publish forky_bevy  			{{args}}	| true
	just publish forky  					{{args}}	| true
#just publish forky_play 			| true
# just publish forky_ai 				| true

start crate: 
	./target/debug/{{crate}}.exe

ci:
	just test-all
	just test-all-wasm

# cargo run -p forky_play	--example test_forky_play	--features sweet/bevy -- --parallel
test-all *args:
	cargo test -p forky_core
	cargo test -p forky_cli
	cargo test -p forky_fs
	cargo test -p forky_bevy
	cargo test -p forky_net
	cargo test -p forky_core --target wasm32-unknown-unknown
	cargo test -p forky_bevy --target wasm32-unknown-unknown
	cargo test -p forky_web --target wasm32-unknown-unknown

test-ci crate *args:
	RUST_BACKTRACE={{backtrace}} cargo test -p {{crate}} --lib {{features}} -- {{args}}

test crate *args:
	just watch just test-ci {{crate}} -w {{args}}

doc-w crate *args:
  echo "Navigate to the crate, ie http://127.0.0.1:3000/sweet"
  forky serve ./target/doc & just watch just doc {{crate}} {{args}}
doc crate *args:
  cargo doc --all-features -p {{crate}} {{args}}
# RUSTDOCFLAGS='--show-coverage -Z unstable-options' cargo doc  -p {{crate}} {{args}}

book:
	cd docs && mdbook serve

watch *command:
	forky watch --rusty	-- {{command}}
### PLAY ###

vis-w crate example:
	just vis-serve & just watch just vis {{crate}} {{example}}

vis crate example:
	just run {{crate}} {{example}}
	just dot-to-svg target/graph/render_graph.dot

vis-serve:
	cd ./target/graph && forky serve

# requires https://graphviz.org/download/
dot-to-svg target:
	dot -Tsvg -O {{target}}

bevy-deps:
	cargo search bevy
	cargo search bevy-inspector-egui 
	cargo search bevy_mod_debugdump 
	cargo search bevy_rapier3d 


### WASM ###

# run-wasm crate example:
# 	cargo run -p {{crate}} --example {{example}} --target wasm32-unknown-unknown

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
	openssl req -new -key target/client-key.pem -subj "/CN=foobar.com\/emailAddress=admin@foobar.com/C=US/ST=Ohio/L=Columbus/O=Widgets Inc/OU=Some Unit" -out target/client.csr
	openssl x509 -req -in target/client.csr -signkey target/client-key.pem -out target/client-cert.pem

style:
	cargo run -p forky_cli style all

watch-css crate *args:
	forky watch \
	just build-css {{crate}} {{args}} \
	-w '**/*.css' \
	-i '{.git,target,html}/**' \

@build-css crate *args:
	just lightning ./crates/{{crate}}/src/style/index.css ./html/style.css {{args}}

lightning in out *args:
	npx lightningcss {{in}} --bundle -m -o {{out}} {{args}}

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
