# Bevy WebXR

VERY early-stage standalone WebXR renderer for the Bevy Engine.

## Quickstart
### Terminal 1.
I'm using npm `live-server` because I know how to do https (required for webxr) with that but feel free to use whatever you like.
```sh
just ssl
just serve
```
### Terminal 2
```sh
just build-w bevy_webxr
```

### Code

```rs
use bevy::prelude::*;
use bevy_webxr::{demo::DemoScenePlugin, bevy_utils::WebXrPlugin};

fn main() {
	let mut app = App::new();
	app
		.add_plugins(DemoScenePlugin)
		.add_plugins(WebXrPlugin)
	// custom run method hooks into session.requestAnimationFrame
	app.run_webxr();
}
```

### Non-rust Dependencies
These are all pretty much dev's choice for running commands, servers etc. but this is how I do it.

- [just](https://github.com/casey/just)
	- Command runner
	- Alternatively copy/paste commands from [justfile](./justfile)
- [cygwin](https://www.cygwin.com/)
	- Bash shell for Windows
- [live-server](https://www.npmjs.com/package/live-server)
	- feel free to use any server you're used to
- [openssl](https://www.openssl.org/)
	- https cert key generation

- not required, but good idea to use the [WebXR API Emulator](https://chrome.google.com/webstore/detail/webxr-api-emulator/mjddjgeghkdijejnciaefnkjmkafnnje?hl=en) for testing

see [CONTRIBUTING](contributing.md) for more details about the repo

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
### Contributing

- I know this is a little unconventional, but I auto-generate `mod.rs` files based on directory layout. The default rules are that any file and directory with a `_` prefix is exposed as-is. Otherwise directories are treated as 'namespaces'. The `lib.rs` file is not auto-generated 
## TODO
- use bevy/hashbrown hashmaps for performance