#define_import_path forky::utility

const PI: f32 = 3.14159265358979323846264338327950288;
const TAU: f32 = 6.28318530717958647692528676655900577;

// shader art: https://youtu.be/f4s1h2YETNY

// Palette
// how it works https://iquilezles.org/articles/palettes/
// color tool http://dev.thi.ng/gradients/
fn palette(
	dc_offset:vec3<f32>,
	amp:vec3<f32>,
	freq:vec3<f32>,
	phase:vec3<f32>,
	t:f32)->vec3<f32>{
	return dc_offset + cos(TAU * (t + phase * freq)) * amp;
}

// falloff should be low value, ie 0.01
fn bloom(value:f32,falloff:f32,contrast:f32)->f32{
	return pow(falloff / value, contrast);
}