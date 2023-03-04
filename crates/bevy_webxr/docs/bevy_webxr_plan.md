# Bevy WebXR Plan

[x] upgrade bevy
[x] web_sys hello world
[X] web_sys webxr
[X] bevy web_sys hello world
[X] bevy webxr inline
[] bevy webxr immersive
[] ...
[] profit

## Notes

- Looks like neither winit nor bevy touch webgl and just put pixels, may need to handroll this. check out wasm-bindgen webgl example

## Reference

- WGPU alternate framebuffers
	- https://github.com/gfx-rs/wgpu/pull/2609
	- demo: 
		- https://github.com/expenses/kiss-engine/blob/d877a28c0bf86e3b65fb438446647e2998820be4/ludum-dare/src/main.rs
		- https://expenses.github.io/wgpu-vr/
	- apparently also uses it: https://github.com/MeetKai/superconductor/

- SketchPunk Labs
	- https://www.youtube.com/watch?v=LtFujAtKM5I&list=PLMinhigDWz6emRKVkVIEAaePW7vtIkaIF
- Samples
	- https://immersive-web.github.io/webxr-samples/
- Winit run return
	- https://github.com/rust-windowing/winit/issues/1493