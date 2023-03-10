#import bevy_core_pipeline::fullscreen_vertex_shader

@group(0) @binding(0)
var my_texture: texture_2d<f32>;
@group(0) @binding(1)
var my_sampler: sampler;

@fragment
fn fragment(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
	// return vec4(1.,1.,0.,1.);
	let uv_inv = vec2(uv.x, 1. - uv.y);
	// return vec4(uv.x,uv_inv.y,0.,1.);
		return textureSample(my_texture, my_sampler, uv_inv);
}
