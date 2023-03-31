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
use bevy_webxr::{demo::DemoScenePlugin, bevy_utils::WebXrPlugin};

fn main() {
	let mut app = App::new();
	app
		.add_plugin(DemoScenePlugin)
		.add_plugin(WebXrPlugin)
	// custom run method hooks into session.requestAnimationFrame
	app.run_webxr();
}
```

## Resources

- JsValues
	- `XrFrame`
	- `XrReferenceSpace`
	- `XrReferenceSpaceType`
	- `XrSession`
	- `XrSessionMode`
	- `XrWebGlLayer`
- Additional
	- `BevyXrViewLookup`
	- `BevyInputSourceLookup`
	- `FramebufferTextureViewId`

JsValue Resources can be accesed via `NonSend`, ie:

```rs
fn do_thing(frame: NonSend<web_sys::XrFrame>){
	...
}
```

## Components

- `BevyXrView`
	- Added to each camera
	- `LeftCamera`/`RightCamera`
- `BevyXrInputSource`
	- Added to each controller/hand
- Handedness
	- Used for controllers and views
	- `LeftHandedness`
	- `RightHandedness`
	- `NoneHandedness`

## Issues

- Lighting seems about 10x darker than regular rendering
- Oculus Quest
	- Standard Materials broken on Android/Quest web, [issue](https://github.com/bevyengine/bevy/issues/4582)
		- Thus controllers currently dont render
	- **Help Wanted** - Subtle issue with projection matrix, ie rotating head seems to translate meshes slightly
- No other devices have been tested

## TODO
- use bevy/hashbrown hashmaps for performance

## Contributing

- For more info about the repo see [CONTRIBUTING](contributing.md)
- I've published this because I figure an early stage crate is better than nothing. If you are also working on bevy webxr and/or want to talk about crate transfer get in touch!