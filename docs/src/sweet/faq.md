# FAQ


### Why use `[[example]]` instead of `[[test]]`
This makes it easier for the wasm test runner to produce cleaner output, but if you're only running native tests feel free to use `[[test]]` with `harness=false`.

### What about wasm-bindgen-test?
Sweet has different priorities from [wasm-bindgen-test](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html) in its current state, namely a focus on UI & interactivity.
