// #import forky::utility
#import forky::sdf2

@group(1) @binding(0)
var<uniform> color: vec4<f32>;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
	// return get_color();
	let point = uv * 2. - 1.;
	var val = sdCircle(point,0.5);
	val = abs(val);
	val = val * 0.5 + 0.5;
	// val *= 1.8;
	// val = val * val * val * val;
	return vec4(val,val,val,1.);
  // return color;
}
