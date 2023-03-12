# Bevy WebXR

A Standalone WebXR renderer for the Bevy Engine. Tested on Oculus Quest 2.

## Quickstart

- just serve
- just build-w bevy_webxr

## Resources

- `XrFrame`
- `XrReferenceSpace`
- `XrSession`
- `XrSessionMode`
- `XrWebGlLayer`

Resources can be accesed as a `NonSend`, ie:

```rs
fn do_thing(frame: NonSend<web_sys::XrFrame>){
	...
}
```

## Issues

- Lighting is very dark
- Standard Materials broken on Android Chrome, [issue](https://github.com/bevyengine/bevy/issues/4582)