
@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
		return vec4(uv.x,uv.y,1.,1.);
}