# Bevy WebXR


A proof-of-concept for WebXR in Bevy `0.11.0`. Visit this page in your Oculus Quest to see it in action!

<div style="text-align:center">
	<iframe src="../../demos/bevy-webxr" width="80%" height="500px"/></iframe>
</div>

## Issues

- Lighting seems about 10x darker than regular rendering
- Oculus Quest
	- Standard Materials broken on Android/Quest web, [issue](https://github.com/bevyengine/bevy/issues/4582)
		- Thus controllers currently dont render
	- **Help Wanted** - Subtle issue with projection matrix, ie tilting head on z axis seems to translate meshes slightly
- No other devices have been tested

The source is pretty messy and im using a few non-conventional build steps so only try running locally if you're feeling brave.
That said it may serve as a reference for anybody trying to implement their own webxr crates.

## Usage

If you *are* feeling brave here are some notes:

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
