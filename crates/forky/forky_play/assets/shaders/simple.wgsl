#import bevy_core_pipeline::fullscreen_vertex_shader

@group(0) @binding(0)
var<uniform> color: vec4<f32>;
@group(0) @binding(1)
var my_texture: texture_2d<f32>;
@group(0) @binding(2)
var my_sampler: sampler;

@fragment
fn fragment(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
	// return color;
		return color * textureSample(my_texture, my_sampler, uv);
	// textureSample(texture, sampler, uv)
		// return vec4(uv.xy,0.,1.);
		// return vec4(color.r,color.g ,color.b,1.);
}