# Bevy WebXR

A Standalone WebXR renderer for the Bevy Engine. Tested on Oculus Quest 2.

## Quickstart

- just serve
- just build-w bevy_webxr

## Method

Two cameras render to an image, and the `blit_node` will run a simple shader that writes the image to the framebuffer.

## Issues

- Lighting is very dark
- Standard Materials broken on Android Chrome, [issue](https://github.com/bevyengine/bevy/issues/4582)