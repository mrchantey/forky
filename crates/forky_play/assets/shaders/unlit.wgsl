@group(1) @binding(0)
var<uniform> color: vec4<f32>;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    return color;
}
