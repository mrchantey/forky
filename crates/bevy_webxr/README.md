# Bevy WebXR

VERY early-stage standalone WebXR renderer for the Bevy Engine.

## Quickstart

### Dependencies

- currently requires [RenderTarget::TextureView](https://github.com/bevyengine/bevy/pull/8042). The PR is scheduled for `bevy 0.11`
- as standard materials are currently not supported, you'll need to copy `assets/shaders` into the `html` output directory
- not required, but good idea to use the [WebXR API Emulator](https://chrome.google.com/webstore/detail/webxr-api-emulator/mjddjgeghkdijejnciaefnkjmkafnnje?hl=en) for testing
- see [CONTRIBUTING](contributing.md) for more details about the repo

### Code

```rs
use bevy::prelude::*;
use bevy_webxr::{demo, bevy_utils};

fn main() {
	let mut app = App::new();
	app
		.add_plugin(demo::DemoScenePlugin)
		.add_plugin(bevy_utils::WebXrPlugin {
			..default()
		});
	// custom run method hooks into session.requestAnimationFrame
	app.run_webxr();
}
```

## Resources

- `XrFrame`
- `XrReferenceSpace`
- `XrSession`
- `XrSessionMode`
- `XrWebGlLayer`
- `NonSend<BevyXrView>` TODO xrviewlookup
- `BevyInputSourceLookup`

JsValue Resources can be accesed via `NonSend`, ie:

```rs
fn do_thing(frame: NonSend<web_sys::XrFrame>){
	...
}
```
## Issues

- All devices
	- Lighting seems about 10x darker than regular rendering
	- Controller models are inverted on x axis
- [WebXR API Emulator](https://chrome.google.com/webstore/detail/webxr-api-emulator/mjddjgeghkdijejnciaefnkjmkafnnje?hl=en)
- Oculus Quest
	- Standard Materials broken on Android/Quest web, [issue](https://github.com/bevyengine/bevy/issues/4582)
		- Thus controllers currently dont render
	- Projection seems a bit off
- No other devices have been tested

## Contributing

- For more info about the repo see [CONTRIBUTING](contributing.md)
- I've published this because I figure an early stage crate is better than nothing. If you are also working on bevy webxr and/or want to talk about crate transfer get in touch!