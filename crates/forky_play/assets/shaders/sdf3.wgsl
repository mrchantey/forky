#define_import_path forky::sdf3
/*
https://gist.github.com/munrocket/f247155fc22ecb8edf974d905c677de1

# WGSL 3D SDF Primitives

#### How to use this gist:
1. Build a sphere tracer with WebGPU ([paper](https://www.researchgate.net/publication/2792108_Sphere_Tracing_A_Geometric_Method_for_the_Antialiased_Ray_Tracing_of_Implicit_Surfaces), [paper2](https://www.semanticscholar.org/paper/Enhanced-Sphere-Tracing-Keinert-Sch%C3%A4fer/4c9bd91bd044980f5746d623315be5285cc799c9), [youtube](https://www.youtube.com/watch?v=PGtv-dBi2wE))
2. Create model with sdf functions from here
3. Add light and shadows
4. ???
5. PROFIT

This code tested in Chrome and Firefox, should work on PC too.

Function `select` can work differently in browsers.

# Primitives

#### Sphere - exact
*/
fn sdSphere(p: vec3<f32>, r: f32) -> f32 {
  return length(p) - r;
}
/*
<img src="https://habrastorage.org/webt/9s/1j/gw/9s1jgwsvfhr9-q0v1n1xpgozm3i.jpeg" width="150px">

#### Ellipsoid - bound (not exact)
*/
fn sdEllipsoid(p: vec3<f32>, r: vec3<f32>) -> f32 {
  let k0 = length(p / r);
  let k1 = length(p / (r * r));
  return k0 * (k0 - 1.) / k1;
}
/*
<img src="https://habrastorage.org/webt/zz/fd/fw/zzfdfwtf4mmb0db7mzpwr0kyoqa.jpeg" width="150px">

#### Box - exact
*/
fn sdBox(p: vec3<f32>, b: vec3<f32>) -> f32 {
  let q = abs(p) - b;
  return length(max(q, vec3<f32>(0.))) + min(max(q.x, max(q.y, q.z)), 0.);
}
/*
<img src="https://habrastorage.org/webt/j0/jp/rp/j0jprpokpxonzzpiz9zdm1qh2os.jpeg" width="150px">

#### Round Box - exact
*/
fn sdRoundBox(p: vec3<f32>, b: vec3<f32>, r: f32) -> f32 {
  let q = abs(p) - b;
  return length(max(q, vec3<f32>(0.))) + min(max(q.x,max(q.y, q.z)), 0.) - r;
}
/*
<img src="https://habrastorage.org/webt/vf/g0/ze/vfg0zewp6rc61gnwnextb8v3lo4.jpeg" width="150px">

#### Box Frame - exact
*/
fn sdBoxFrame(p: vec3<f32>, b: vec3<f32>, e: f32) -> f32 {
  let q = abs(p) - b;
  let w = abs(q + e) - e;
  return min(min(
      length(max(vec3<f32>(q.x, w.y, w.z), vec3<f32>(0.))) + min(max(q.x, max(w.y, w.z)), 0.),
      length(max(vec3<f32>(w.x, q.y, w.z), vec3<f32>(0.))) + min(max(w.x, max(q.y, w.z)), 0.)),
      length(max(vec3<f32>(w.x, w.y, q.z), vec3<f32>(0.))) + min(max(w.x, max(w.y, q.z)), 0.));
}
/*
<img src="https://habrastorage.org/webt/tx/7v/gb/tx7vgburvrj0rlrgsgxea1c0dni.jpeg" width="150px">

#### Gyroid - bound
*/
fn sdGyroid(p: vec3<f32>, h: f32) -> f32 {
  return abs(dot(sin(p), cos(p.zxy))) - h;
}
/*
<img src="https://habrastorage.org/webt/nu/mi/v9/numiv9d7cz6dfxrnkgripzbbai4.jpeg" width="150px">

#### Torus - exact
*/
fn sdTorus(p: vec3<f32>, R: f32, r: f32) -> f32 {
  let q = vec2<f32>(length(p.xz) - R, p.y);
  return length(q) - r;
}
/*
<img src="https://habrastorage.org/webt/7j/lz/fi/7jlzfitp9knoablnx38najyz8hu.jpeg" width="150px">

#### Capped Torus - exact
*/
fn sdCappedTorus(p: vec3<f32>, R: f32, r: f32, sincos: vec2<f32>) -> f32 {
  let q = vec3<f32>(abs(p.x), p.y, p.z);
  let k = select(length(q.xy), dot(q.xy, sincos), sincos.y * q.x > sincos.x * q.y);
  return sqrt(dot(q, q) + R * R - 2. * R * k) - r;
}
/*
<img src="https://habrastorage.org/webt/lr/gf/mw/lrgfmwb0yfwaipf1rj3_vzu-tzm.jpeg" width="150px">

#### Link - exact
*/
fn sdLink(p: vec3<f32>, R: f32, r: f32, le: f32) -> f32 {
  let q = vec3<f32>(p.x, max(abs(p.y) - le, 0.), p.z);
  return length(vec2<f32>(length(q.xy) - R, q.z)) - r;
}
/*
<img src="https://habrastorage.org/webt/fk/du/dh/fkdudhqfxcgl3m7fylehvsetnqa.jpeg" width="150px">

#### Capsule / Line - exact
*/
fn sdCapsule(p: vec3<f32>, a: vec3<f32>, b: vec3<f32>, r: f32) -> f32 {
  let pa = p - a;
  let ba = b - a;
  let h = clamp(dot(pa, ba) / dot(ba, ba), 0., 1.);
  return length(pa - ba * h) - r;
}
/*
<img src="https://habrastorage.org/webt/o6/ei/u3/o6eiu3wxukv8jw-owmgegdy27u4.jpeg" width="150px">

#### Vertical Capsule / Line - exact
*/
fn sdVerticalCapsule(p: vec3<f32>, h: f32, r: f32) -> f32 {
  let q = vec3<f32>(p.x, p.y - clamp(p.y, 0., h), p.z);
  return length(q) - r;
}
/*
<img src="https://habrastorage.org/webt/0l/nd/ot/0lndot80hvqtbnhh_6-ncloz7io.jpeg" width="150px">

#### Cylinder - exact
*/
fn sdCylinder(p: vec3<f32>, a: vec3<f32>, b: vec3<f32>, r: f32) -> f32 {
  let ba = b - a;
  let pa = p - a;
  let baba = dot(ba, ba);
  let paba = dot(pa, ba);
  let x = length(pa * baba - ba * paba) - r * baba;
  let y = abs(paba - baba * 0.5) - baba * 0.5;
  let x2 = x * x;
  let y2 = y * y * baba;
  let d = x2 * step(0., x) + y2 * step(0., y);
  let d2 = select(d, -min(x2, y2), max(x, y) < 0.);
  return sign(d2) * sqrt(abs(d2)) / baba;
}
/*
<img src="https://habrastorage.org/webt/33/v-/xz/33v-xz4gqpq_hy2v01mreala19u.jpeg" width="150px">

#### Vertical Cylinder - exact
*/
fn sdVerticalCylinder(p: vec3<f32>, h: f32, r: f32) -> f32 {
  let d = abs(vec2<f32>(length(p.xz), p.y)) - vec2<f32>(r, h);
  return min(max(d.x, d.y), 0.) + length(max(d, vec2<f32>(0.)));
}
/*
<img src="https://habrastorage.org/webt/wr/lx/lq/wrlxlqcrexycg7fkfiygip_18u8.jpeg" width="150px">

#### Rounded Cylinder - exact
*/
fn sdRoundedCylinder(p: vec3<f32>, h: f32, r: f32, re: f32) -> f32 {
  let d = vec2<f32>(length(p.xz) - 2. * r + re, abs(p.y) - h);
  return min(max(d.x, d.y), 0.) + length(max(d, vec2<f32>(0.))) - re;
}
/*
<img src="https://habrastorage.org/webt/d9/qs/3w/d9qs3wpn4bqozwi0vsgqw8xqps8.jpeg" width="150px">

#### Infinite Cylinder - exact
*/
fn sdInfiniteCylinder(p: vec3<f32>, c: vec3<f32>) -> f32 {
  return length(p.xz - c.xy) - c.z;
}
/*
<img src="https://habrastorage.org/webt/bo/-p/pf/bo-ppfbsp71n1srnrclo3qc9tby.jpeg" width="150px">

#### Cone - exact
*/
fn sdCone(p: vec3<f32>, h: f32, sincos: vec2<f32>) -> f32 {
  // Alternatively pass q instead of (sin(alpha), cos(alpha))
  let q = h * vec2<f32>(sincos.x / sincos.y, -1.);

  let w = vec2<f32>(length(p.xz), p.y);
  let a = w - q * clamp(dot(w,q) / dot(q,q), 0., 1.);
  let b = w - q * vec2<f32>(clamp(w.x / q.x, 0., 1.), 1.);
  let k = sign(q.y);
  let d = min(dot(a, a), dot(b, b));
  let s = max(k * (w.x * q.y - w.y * q.x), k * (w.y - q.y));
  return sqrt(d) * sign(s);
}
/*
<img src="https://habrastorage.org/webt/ie/lz/1h/ielz1hqyxdwhft2rt-x6333c478.jpeg" width="150px">

#### Cone - bound (not exact)
*/
fn sdConeBound(p: vec3<f32>, h: f32, sincos: vec2<f32>) -> f32 {
  return max(dot(sincos.yx, vec2<f32>(length(p.xz), p.y)), -h - p.y);
}
/*
<img src="https://habrastorage.org/webt/ie/lz/1h/ielz1hqyxdwhft2rt-x6333c478.jpeg" width="150px">

#### Infinite Cone - exact
*/
fn sdInfiniteCone(p: vec3<f32>, sincos: vec2<f32>) -> f32 {
  let q = vec2<f32>(length(p.xz), -p.y);
  let d = length(q - sincos * max(dot(q, sincos), 0.));
  return d * select(-1., 1., q.x * sincos.y - q.y * sincos.x > 0.0);
}
/*
<img src="https://habrastorage.org/webt/39/j5/fs/39j5fsjq-6onu2lxbwtakun5g1k.jpeg" width="150px">

#### Capped Vertical Cone - exact
*/
fn sdCappedVerticalCone(p: vec3<f32>, h: f32, r1: f32, r2: f32) -> f32 {
  let q = vec2<f32>(length(p.xz), p.y);
  let k1 = vec2<f32>(r2, h);
  let k2 = vec2<f32>(r2 - r1, 2. * h);
  let ca = vec2<f32>(q.x - min(q.x, select(r2, r1, q.y < 0.)), abs(q.y) - h);
  let cb = q - k1 + k2 * clamp(dot(k1 - q, k2) / dot(k2, k2), 0., 1.);
  let s = select(1., -1., cb.x < 0. && ca.y < 0.);
  return s * sqrt(min(dot(ca, ca), dot(cb, cb)));
}
/*
<img src="https://habrastorage.org/webt/p4/br/qq/p4brqq1bibmhil4njb9v8bjwb8s.jpeg" width="150px">

#### Capped Cone - exact
*/
fn sdCappedCone(p: vec3<f32>, a: vec3<f32>, b: vec3<f32>, ra: f32, rb: f32) -> f32 {
  let rba = rb - ra;
  let baba = dot(b - a, b - a);
  let papa = dot(p - a, p - a);
  let paba = dot(p - a, b - a) / baba;
  let x = sqrt(papa - paba * paba * baba);
  let cax = max(0.0, x - select(rb, ra, paba < 0.5));
  let cay = abs(paba - 0.5) - 0.5;
  let k = rba * rba + baba;
  let f = clamp((rba * (x - ra) + paba * baba) / k, 0.0, 1.0);
  let cbx = x - ra - f * rba;
  let cby = paba - f;
  let s = select(1., -1., cbx < 0.0 && cay < 0.0);
  return s * sqrt(min(cax * cax + cay * cay * baba, cbx * cbx + cby * cby * baba));
}
/*
<img src="https://habrastorage.org/webt/ch/7i/gv/ch7igvrlcmjtn38qciwdzpwckfg.jpeg" width="150px">

#### Round Vertical cone - exact
*/
fn sdRoundVerticalCone(p: vec3<f32>, h: f32, r1: f32, r2: f32) -> f32 {
  let q = vec2<f32>(length(p.xz), p.y);
  let b = (r1 - r2) / h;
  let a = sqrt(1. - b * b);
  let k = dot(q, vec2<f32>(-b, a));
  if (k < 0.) { return length(q) - r1; }
  if (k > a * h) { return length(q - vec2<f32>(0., h)) - r2; }
  return dot(q, vec2<f32>(a, b)) - r1;
}
/*
<img src="https://habrastorage.org/webt/rv/ww/-1/rvww-1s4txpfkheptynmb9m2jyg.jpeg" width="150px">

#### Round cone - exact
*/
fn sdRoundCone(p: vec3<f32>, a: vec3<f32>, b: vec3<f32>, r1: f32, r2: f32) -> f32 {
  let ba = b - a;
  let l2 = dot(ba, ba);
  let rr = r1 - r2;
  let a2 = l2 - rr * rr;
  let il2 = 1. / l2;

  let pa = p - a;
  let y = dot(pa, ba);
  let z = y - l2;
  let w = pa * l2 - ba * y;
  let x2 = dot(w, w);
  let y2 = y * y * l2;
  let z2 = z * z * l2;

  let k = sign(rr) * rr * rr * x2;
  if (sign(z) * a2 * z2 > k) { return sqrt(x2 + z2) * il2 - r2; }
  if (sign(y) * a2 * y2 < k) { return sqrt(x2 + y2) * il2 - r1; }
  return (sqrt(x2 * a2 * il2) + y * rr) * il2 - r1;
}
/*
<img src="https://habrastorage.org/webt/61/ft/aa/61ftaa1ral1hbj_eijczixm0tnq.jpeg" width="150px">

#### Solid Angle - exact
*/
fn sdSolidAngle(p: vec3<f32>, sincos: vec2<f32>, r: f32) -> f32 {
  let q = vec2<f32>(length(p.xz), p.y);
  let l = length(q) - r;
  let m = length(q - sincos * clamp(dot(q, sincos), 0., r));
  return max(l, m * sign(sincos.y * q.x - sincos.x * q.y));
}
/*
<img src="https://habrastorage.org/webt/7z/ee/yu/7zeeyuuhnwcrcpmht-h8nxi0em4.jpeg" width="150px">

#### Plane - exact
*/
fn sdPlane(p: vec3<f32>, n: vec3<f32>, h: f32) -> f32 {
  // n must be normalized
  return dot(p, n) + h;
}
/*
<img src="https://habrastorage.org/webt/sz/cn/8x/szcn8xvstcyyb9irdpehjhlpqpk.jpeg" width="150px">

#### Octahedron - exact
*/
fn sdOctahedron(p: vec3<f32>, s: f32) -> f32 {
  var q: vec3<f32> = abs(p);
  let m = q.x + q.y + q.z - s;
  if (3. * q.x < m) {q = q.xyz;}
  else {if (3. * q.y < m) {q = q.yzx;}
        else {if (3. * q.z < m) {q = q.zxy;}
              else {return m * 0.57735027;}}}
  let k = clamp(0.5 * (q.z - q.y + s), 0., s);
  return length(vec3<f32>(q.x, q.y - s + k, q.z - k));
}
/*
<img src="https://habrastorage.org/webt/pt/eg/rt/ptegrtffv6andxzppbgkzpl8poe.jpeg" width="150px">

#### Octahedron - bound (not exact)
*/
fn sdOctahedronBound(p: vec3<f32>, s: f32) -> f32 {
  let q = abs(p);
  return (q.x + q.y + q.z - s) * 0.57735027;
}
/*
<img src="https://habrastorage.org/webt/pt/eg/rt/ptegrtffv6andxzppbgkzpl8poe.jpeg" width="150px">

#### Pyramid - exact
*/
fn sdPyramid(p: vec3<f32>, h: f32) -> f32 {
  let m2 = h * h + 0.25;
  var xz: vec2<f32> = abs(p.xz);
  xz = select(xz, xz.yx, xz[1] > xz[0]);
  xz = xz - vec2<f32>(0.5);

  let q = vec3<f32>(xz[1], h * p.y - 0.5 * xz[0], h * xz[0] + 0.5 * p.y);
  let s = max(-q.x, 0.);
  let t = clamp((q.y - 0.5 * xz[1]) / (m2 + 0.25), 0., 1.);

  let a = m2 * (q.x + s) * (q.x + s) + q.y * q.y;
  let b = m2 * (q.x + 0.5 * t) * (q.x + 0.5 * t) + (q.y - m2 * t) * (q.y - m2 * t);

  let d2 = min(a, b) * step(min(q.y, -q.x * m2 - q.y * 0.5), 0.);
  return sqrt((d2 + q.z * q.z) / m2) * sign(max(q.z, -p.y));
}
/*
<img src="https://habrastorage.org/webt/g2/xf/gc/g2xfgc02jc-ty5mlqgyncutoyss.jpeg" width="150px">

#### Hexagonal Prism - exact
*/
fn sdHexPrism(p: vec3<f32>, h: vec2<f32>) -> f32 {
  let k = vec3<f32>(-0.8660254, 0.5, 0.57735);
  let a = abs(p);
  let v = a.xy - 2. * min(dot(k.xy, a.xy), 0.) * k.xy;
  let d1 = length(v - vec2<f32>(clamp(v.x, -k.z * h.x, k.z * h.x), h.x)) * sign(v.y - h.x);
  let d2 = a.z - h.y;
  return min(max(d1, d2), 0.) + length(max(vec2<f32>(d1, d2), vec2<f32>(0.)));
}
/*
<img src="https://habrastorage.org/webt/tz/2i/3n/tz2i3na0oscwx1g_skclcoduo3o.jpeg" width="150px">

#### Triangular Prism - bound
*/
fn sdTriPrism(p: vec3<f32>, h: vec2<f32>) -> f32 {
  let q = abs(p);
  return max(q.z - h.y, max(q.x * 0.866025 + p.y * 0.5, -p.y) - h.x * 0.5);
}
/*
<img src="https://habrastorage.org/webt/zc/2k/2s/zc2k2sm4cudwfsghvk7ibkdztcc.jpeg" width="150px">

#### Quadratic Bezier - exact
*/
fn sdBezier(p: vec3<f32>, A: vec3<f32>, B: vec3<f32>, C: vec3<f32>) -> vec2<f32> {
  let a = B - A;
  let b = A - 2. * B + C;
  let c = a * 2.;
  let d = A - p;
  let kk = 1. / dot(b, b);
  let kx = kk * dot(a, b);
  let ky = kk * (2. * dot(a, a) + dot(d, b)) / 3.;
  let kz = kk * dot(d, a);

  let p1 = ky - kx * kx;
  let p3 = p1 * p1 * p1;
  let q = kx * (2.0 * kx * kx - 3.0 * ky) + kz;
  var h: f32 = q * q + 4. * p3;

  var res: vec2<f32>;
  if (h >= 0.) {
    h = sqrt(h);
    let x = (vec2<f32>(h, -h) - q) / 2.;
    let uv = sign(x) * pow(abs(x), vec2<f32>(1. / 3.));
    let t = clamp(uv.x + uv.y - kx, 0., 1.);
    let f = d + (c + b * t) * t;
    res = vec2<f32>(dot(f, f), t);
  } else {
    let z = sqrt(-p1);
    let v = acos(q / (p1 * z * 2.)) / 3.;
    let m = cos(v);
    let n = sin(v) * 1.732050808;
    let t = clamp(vec2<f32>(m + m, -n - m) * z - kx, vec2<f32>(0.0), vec2<f32>(1.0));
    let f = d + (c + b * t.x) * t.x;
    var dis: f32 = dot(f, f);
    res = vec2<f32>(dis, t.x);

    let g = d + (c + b * t.y) * t.y;
    dis = dot(g, g);
    res = select(res, vec2<f32>(dis, t.y), dis < res.x);
  }
  res.x = sqrt(res.x);
  return res;
}
/*
<img src="https://habrastorage.org/webt/ad/9t/cu/ad9tcutikuqz9yxdrpdquugqkrs.jpeg" width="150px">

#### Triangle - exact
*/
fn udTriangle(p: vec3<f32>, a: vec3<f32>, b: vec3<f32>, c: vec3<f32>) -> f32 {
  let ba = b - a; let pa = p - a;
  let cb = c - b; let pb = p - b;
  let ac = a - c; let pc = p - c;
  let nor = cross(ba, ac);
   let d1 = ba * clamp(dot(ba, pa) / dot(ba, ba), 0., 1.) - pa;
  let d2 = cb * clamp(dot(cb, pb) / dot(cb, cb), 0., 1.) - pb;
  let d3 = ac * clamp(dot(ac, pc) / dot(ac, ac), 0., 1.) - pc;
   let k0 = min(min(dot(d1, d1), dot(d2, d2)), dot(d3, d3));
  let k1 = dot(nor, pa) * dot(nor, pa) / dot(nor, nor);
  let t = sign(dot(cross(ba, nor), pa)) + sign(dot(cross(cb, nor), pb)) +
      sign(dot(cross(ac, nor), pc));
  return sqrt(select(k0, k1, t < 2.));
}
/*
<img src="https://habrastorage.org/webt/jk/cc/eo/jkcceovnen8mfbwdzyu9gpxcxhi.jpeg" width="150px">

 #### Quad - exact
*/
fn udQuad(p: vec3<f32>, a: vec3<f32>, b: vec3<f32>, c: vec3<f32>, d: vec3<f32>) -> f32 {
  let ba = b - a; let pa = p - a;
  let cb = c - b; let pb = p - b;
  let dc = d - c; let pc = p - c;
  let ad = a - d; let pd = p - d;
  let nor = cross(ba, ad);
   let d1 = ba * clamp(dot(ba, pa) / dot(ba, ba), 0., 1.) - pa;
  let d2 = cb * clamp(dot(cb, pb) / dot(cb, cb), 0., 1.) - pb;
  let d3 = dc * clamp(dot(dc, pc) / dot(dc, dc), 0., 1.) - pc;
  let d4 = ad * clamp(dot(ad, pd) / dot(ad, ad), 0., 1.) - pd;
   let k0 = min(min(dot(d1, d1), dot(d2, d2)), min(dot(d3, d3), dot(d4, d4)));
  let k1 = dot(nor, pa) * dot(nor, pa) / dot(nor, nor);
  let t = sign(dot(cross(ba, nor), pa)) + sign(dot(cross(cb, nor), pb)) +
      sign(dot(cross(dc, nor), pc)) + sign(dot(cross(ad, nor), pd));
  return sqrt(select(k0, k1, t < 3.));
}
/*
<img src="https://habrastorage.org/webt/fv/nu/yc/fvnuycg5uvk6bvjthgo8en0_7su.jpeg" width="150px">

# Boolean operations with primitives

#### Union, Subtraction, Intersection - exact (outside), bound, bound
*/
fn opUnion(d1: f32, d2: f32) -> f32 { return min(d1, d2); }

fn opSubtraction(d1: f32, d2: f32) -> f32 { return max(d1, -d2); }

fn opIntersection(d1: f32, d2: f32) -> f32 { return max(d1, d2); }
/*
<img src="https://habrastorage.org/webt/l6/md/vd/l6mdvd5dynh6ewrw3gpmkhsetow.jpeg" width="450px">

#### Chamfer Union, Chamfer Subtraction, Chamfer Intersection - bound, bound, bound
*/
fn opUnionChamfer(d1: f32, d2: f32, r: f32) -> f32 {
  return min(min(d1, d2), (d1 - r + d2) * 0.5);
}

fn opSubtractionChamfer(d1: f32, d2: f32, r: f32) -> f32{
  return max(max(d1, -d2), (d1 + r - d2) * 0.5);
}

fn opIntersectionChamfer(d1: f32, d2: f32, r: f32) -> f32 {
  return max(max(d1, d2), (d1 + r + d2) * 0.5);
}
/*
<img src="https://habrastorage.org/webt/tg/vq/jr/tgvqjr32gtadua4rib-fvfxz4ta.jpeg" width="450px">

#### Blend Union, Blend Subtraction, Blend Intersection - bound, bound, bound
*/
fn opUnionBlend(d1: f32, d2: f32, k: f32) -> f32 {
  let h = clamp(0.5 + 0.5 * (d2 - d1) / k, 0., 1.);
  return mix(d2, d1, h) - k * h * (1. - h);
}

fn opSubtractionBlend(d1: f32, d2: f32, k: f32) -> f32 {
  let h = clamp(0.5 - 0.5 * (d1 + d2) / k, 0., 1.);
  return mix(d1, -d2, h) + k * h * (1. - h);
}

fn opIntersectionBlend(d1: f32, d2: f32, k: f32) -> f32 {
  let h = clamp(0.5 - 0.5 * (d2 - d1) / k, 0., 1.);
  return mix(d2, d1, h) + k * h * (1. - h);
}
/*
<img src="https://habrastorage.org/webt/z_/l0/4g/z_l04gv-0rvvjbfj1gwflukjdqs.jpeg" width="450px">

# Displacement

#### Displacement - bound (not exact)
*/
fn opDisplace(d1: f32, d2: f32) -> f32 {
  return d1 + d2;
}
//let d = opDisplace(sdfPrimitive3d(p), displacement3d(p));
/*
<img src="https://habrastorage.org/webt/7g/zc/vh/7gzcvhiv_dprnk4ohfjltszz510.jpeg" width="150px">

#### Twist - bound
*/
fn opTwist(p: vec3<f32>, k: f32) -> vec3<f32> {
  let s = sin(k * p.y);
  let c = cos(k * p.y);
  let m = mat2x2<f32>(vec2<f32>(c, s), vec2<f32>(-s, c));
  return vec3<f32>(m * p.xz, p.y);
}
//let d = sdfPrimitive3d(opTwist(p, k));
/*
<img src="https://habrastorage.org/webt/9w/-e/5s/9w-e5spb9rqy41hepfwwqrfwd14.jpeg" width="150px">

#### Bend - bound
*/
fn opCheapBend(p: vec3<f32>, k: f32) -> vec3<f32> {
  let s = sin(k * p.x);
  let c = cos(k * p.x);
  let m = mat2x2<f32>(vec2<f32>(c, s), vec2<f32>(-s, c));
  return vec3<f32>(m * p.xy, p.z);
}
//let d = sdfPrimitive3d(opCheapBend(p, k));
/*
<img src="https://habrastorage.org/webt/q4/ej/aq/q4ejaq0uqgw91xr1cbeptz0gmmi.jpeg" width="150px">

# Positioning

#### Translate - exact
*/
fn opTranslate(p: vec3<f32>, t: vec3<f32>) -> vec3<f32> {
  return p - t;
}
//let d = sdfPrimitive3d(opTranslate(p, t));
/*
<img src="https://habrastorage.org/webt/17/20/f7/1720f7wtpzfovqvjrabg4q6qnxu.jpeg" width="150px">

#### 90 degree rotation - exact
*/
fn op90RotateX(p: vec3<f32>) -> vec3<f32> {
  return vec3<f32>(p.x, p.z, -p.y);
}

fn op90RotateY(p: vec3<f32>) -> vec3<f32> {
  return vec3<f32>(-p.z, p.y, p.x);
}

fn op90RotateZ(p: vec3<f32>) -> vec3<f32> {
  return vec3<f32>(p.y, -p.x, p.z);
}
//let d = sdfPrimitive3d(op90RotateZ(p));
/*
<img src="https://habrastorage.org/webt/7h/he/pa/7hhepatlqmihfu7qy8xzkcx5udm.jpeg" width="150px"/>

#### Rotation around axis - exact
*/
fn opRotateX(p: vec3<f32>, a: f32) -> vec3<f32> {
  let s = sin(a); let c = cos(a);
  return vec3<f32>(p.x, c * p.y + s * p.z, -s * p.y + c * p.z);
}

fn opRotateY(p: vec3<f32>, a: f32) -> vec3<f32> {
  let s = sin(a); let c = cos(a);
  return vec3<f32>(c * p.x - s * p.z, p.y, s * p.x + c * p.z);
}

fn opRotateZ(p: vec3<f32>, a: f32) -> vec3<f32> {
  let s = sin(a); let c = cos(a);
  return vec3<f32>(c * p.x + s * p.y, -s * p.x + c * p.y, p.z);
}
//let d = sdfPrimitive3d(opRotateY(p, a));
/*
<img src="https://habrastorage.org/webt/7h/he/pa/7hhepatlqmihfu7qy8xzkcx5udm.jpeg" width="150px"/>

#### Rotation around free axis - exact
*/
fn opRotateE(p: vec3<f32>, e: vec3<f32>, a: f32) -> vec3<f32> {
  let c = cos(a);
  return dot(e, p) * (1. - c) * e - cross(e, p) * sin(a) + c * p;
}
//let d = sdfPrimitive3d(opRotateE(p, normalize(vec3<f32>(1.,0.,.5)), a));
/*
<img src="https://habrastorage.org/webt/dj/ho/sc/djhoscnd2urd2envmjtplgdna7k.jpeg" width="150px"/>

#### Scale - exact
*/
fn opScale(p: vec3<f32>, s: f32) -> vec3<f32> {
  return p / s;
}
//let d = sdfPrimitive3d(opScale(p, s)) * s;
/*
<img src="https://habrastorage.org/webt/p7/ex/r8/p7exr8rg5swkkvofqlgmh4stice.jpeg" width="150px"/>

#### Free transformation - exact
*/
fn opTransform(p: vec3<f32>, transform: mat4x4<f32>) -> vec3<f32> {
  let q = inverse(transform) * vec4<f32>(p, 1.);
}
//let d = sdfPrimitive3d(opTransform(p, transform)) * determinant(transform);

// OR
//let d = sdfPrimitive3d(opScale(opRotateE(opTranslate(p, t), e, a), s)) * s;
/*
<img src="https://habrastorage.org/webt/fp/ub/1x/fpub1xcyvy-mve7l2wvehuk4vym.jpeg" width="150px"/>

#### Symmetry - exact
*/
fn opSymmetryX(p: vec3<f32>) -> vec3<f32> { return vec3<f32>(abs(p.x), p.y, p.z); }

fn opSymmetryY(p: vec3<f32>) -> vec3<f32> { return vec3<f32>(p.x, abs(p.y), p.z); }

fn opSymmetryZ(p: vec3<f32>) -> vec3<f32> { return vec3<f32>(p.x, p.y, abs(p.z)); }
//let d = sdfPrimitive3d(opSymmetryX(p));
/*
<img src="https://habrastorage.org/webt/fp/13/-t/fp13-tocnb3scya_3g_jx7h1s5w.jpeg" width="150px"/>

#### Infinite Repetition - exact
*/
fn opInfArray(p: vec3<f32>, c: vec3<f32>) -> vec3<f32> {
  return p - c * round(p / c);
}
//let d = sdfPrimitive3d(opInfArray(p, c));
/*
<img src="https://habrastorage.org/webt/ly/rj/qv/lyrjqv0xen70howypmxqqprxkyq.jpeg" width="150px"/>

#### Finite Repetition - exact
*/
fn opLimArray(p: vec3<f32>, c: f32, lim: vec3<f32>) -> vec3<f32> {
  return p - c * clamp(round(p / c), -lim, lim);
}
//let d = sdfPrimitive3d(opLimArray(p, c));
/*
<img src="https://habrastorage.org/webt/uo/ii/6f/uoii6fmdcwpacdag8ld88zuu7ne.jpeg" width="150px"/>

# Primitive alterations

#### Elongation - exact
*/
fn opElongate(p: vec3<f32>, h: vec3<f32>) -> vec3<f32> {
  return p - clamp(p, -h, h);
}
//let d = sdfPrimitive3d(opElongateFast(p, h));

fn opElongateCorrect(p: vec3<f32>, h: vec3<f32>) -> vec4<f32> {
  let q = abs(p) - h;
  let sgn = 2. * step(vec3<f32>(0.), p) - vec3<f32>(1.);
  return vec4<f32>(sgn * max(q, vec3<f32>(0.)), min(max(q.x, max(q.y, q.z)), 0.));
}
//let p2 = opElongateCorrect(p, h);
//let d = p2.w + sdfPrimitive3d(p2.xyz);
/*
<img src="https://habrastorage.org/webt/qn/s4/t-/qns4t-dr6gdwdzedebd_ohbsy-a.png" width="150px"/>

#### Rounding - exact
*/
fn opRound(p: vec3<f32>, r: f32) -> f32 {
  return sdfPrimitive3d(p) - r;
}
/*
<img src="https://habrastorage.org/webt/qt/mc/m7/qtmcm7tvpvidagqxnttvbluobbc.jpeg" width="150px"/>

#### Onion - exact
*/
fn opOnion(d: f32, thickness: f32) -> f32 {
  return abs(d) - thickness;
}
//let d = opOnion(sdfPrimitive3d(p), thickness);
/*
<img src="https://habrastorage.org/webt/ja/q9/i0/jaq9i0oqkmdtzya0j9ewyntmth8.jpeg" width="150px"/>

#### Extrusion from 2D SDF - exact
*/
fn opExtrusion(d: f32, z: f32, h: f32) -> f32 {
  let w = vec2<f32>(d, abs(z) - h);
  return min(max(w.x, w.y), 0.) + length(max(w, vec2<f32>(0.)));
}
//let d = opExtrusion(sdfPrimitive2d(p.xy), p.z, h));
/*
<img src="https://habrastorage.org/webt/u_/zq/pm/u_zqpmkpzepemz5fthwhx4f0zy8.jpeg" width="150px"/>

#### Revolution from 2D SDF - exact
*/
fn opRevolution(p: vec3<f32>, o: f32) -> vec2<f32> {
  return vec2<f32>(length(p.xz) - o, p.y);
}
//let d = sdfPrimitive2d(opRevolution(p, h));
/*
<img src="https://habrastorage.org/webt/qd/bn/kw/qdbnkwkkjuqqemdqm-knf5ov53c.jpeg" width="150px"/>

#### Change metric - bound
*/
fn length4(p: vec3<f32>) -> f32 {
  var q: vec3<f32> = p * p;
  q = q * q;
  return sqrt(sqrt(q.x + q.y + q.z));
}

fn length6(p: vec3<f32>) -> f32 {
  var q: vec3<f32> = p * p * p;
  q = q * q;
  return pow(q.x + q.y + q.z, 1. / 6.);
}

fn length8(p: vec3<f32>) -> f32 {
  var q: vec3<f32> = p * p;
  q = q * q; q = q * q;
  return pow(q.x + q.y + q.z, 1. / 8.);
}
/*
<img src="https://habrastorage.org/webt/uw/dm/oz/uwdmozwn25o_dpcdlsrdb-ppevm.jpeg" width="150px"/>

MIT License. © 2020 Inigo Quilez, Johann Korndörfer, Martijn Steinrucken, Munrocket

*/