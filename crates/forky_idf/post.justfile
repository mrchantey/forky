post-hello:
	curl http://192.168.86.24/data \
	-X POST \
	-d hello
post-wasm target:
	curl http://192.168.86.24/data \
	-X POST \
	--data-binary @../wasm_{{target}}/target/wasm32-unknown-unknown/release/wasm_{{target}}.wasm