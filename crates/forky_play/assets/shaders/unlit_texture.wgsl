#import bevy_pbr::forward_io::VertexOutput

@group(1) @binding(0)
var<uniform> color: vec4<f32>;
@group(1) @binding(1)
var<uniform> tiling: vec4<f32>;
@group(1) @binding(2)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(3)
var base_color_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
		let tiled_uv = (in.uv * tiling.xy) % 1.;
    return color * textureSample(base_color_texture, base_color_sampler, tiled_uv);
}
