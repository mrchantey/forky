#define_import_path forky::sdf2
/*
https://gist.github.com/munrocket/30e645d584b5300ee69295e54674b3e4

# WGSL 2D SDF Primitives
Revision: 31.01.2022

#### Circle - exact
*/
fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
  return length(p) - r;
}
/*
<img src="https://i.imgur.com/9Lvlq8U.png" width="150px">

#### Rounded Box - exact
*/
fn sdRoundedBox(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
  var x = r.x;
  var y = r.y;
  x = select(r.z, r.x, p.x > 0.);
  y = select(r.w, r.y, p.x > 0.);
  x  = select(y, x, p.y > 0.);
  let q = abs(p) - b + x;
  return min(max(q.x, q.y), 0.) + length(max(q, vec2<f32>(0.))) - x;
}
/*
<img src="https://i.imgur.com/GuwW2GL.png" width="150px">

#### Box - exact
*/
fn sdBox(p: vec2<f32>, b: vec2<f32>) -> f32 {
  let d = abs(p) - b;
  return length(max(d, vec2<f32>(0.))) + min(max(d.x, d.y), 0.);
}
/*
<img src="https://i.imgur.com/1XJUz6c.png" width="150px">

#### Oriented Box - exact
*/
fn sdOrientedBox(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>, th: f32) -> f32 {
  let l = length(b - a);
  let d = (b - a) / l;
  var q = p - (a + b) * 0.5;
  q = q * mat2x2<f32>(vec2<f32>(d.x, d.y), vec2<f32>(-d.y, d.x));
  q = abs(q) - vec2<f32>(l, th) * 0.5;
  return length(max(q, vec2<f32>(0.))) + min(max(q.x, q.y), 0.);
}
/*
<img src="https://i.imgur.com/trpaYGv.png" width="150px">

#### Segment - exact
*/
fn sdSegment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
  let pa = p - a;
  let ba = b - a;
  let h = clamp(dot(pa, ba) / dot(ba, ba), 0., 1.);
  return length(pa - ba * h);
}
/*
<img src="https://i.imgur.com/fZCTcze.png" width="150px">

#### Rhombus - exact
*/
fn sdRhombus(p: vec2<f32>, b: vec2<f32>) -> f32 {
  let q = abs(p);
  let qb = dot(q, vec2<f32>(b.x, -b.y));
  let bb = dot(b, vec2<f32>(b.x, -b.y));
  let h = clamp((-2. * qb + bb) / dot(b, b), -1., 1.);
  let d = length(q - 0.5 * b * vec2<f32>(1. - h, 1. + h));
  return d * sign(q.x * b.y + q.y * b.x - b.x * b.y);
}
/*
<img src="https://i.imgur.com/ETLVSJO.png" width="150px">

#### Isosceles Trapezoid - exact
*/
fn sdTrapezoid(p: vec2<f32>, r1: f32, r2: f32, he: f32) -> f32 {
  let k1 = vec2<f32>(r2, he);
  let k2 = vec2<f32>(r2 - r1, 2. * he);
  let q = vec2<f32>(abs(p.x), p.y);
  let ca = vec2<f32>(q.x - min(q.x, select(r2, r1, q.y < 0.0)), abs(q.y) - he);
  let cb = q - k1 + k2 * clamp(dot(k1 - q, k2) / dot(k2, k2), 0., 1.);
  let s = select(1., -1., cb.x < 0.0 && ca.y < 0.0);
  return s * sqrt(min(dot(ca, ca), dot(cb, cb)));
}
/*
<img src="https://i.imgur.com/oTElx6G.png" width="150px">

#### Parallelogram - exact
*/
fn sdParallelogram(p: vec2<f32>, wi: f32, he: f32, sk: f32) -> f32 {
  let e = vec2<f32>(sk, he);
  var q: vec2<f32> = select(p, -p, p.y < 0.);
  // horizontal edge
  var w: vec2<f32> = q - e;
  w.x = w.x - clamp(w.x, -wi, wi);
  var d: vec2<f32> = vec2<f32>(dot(w, w), -w.y);
  // vertical edge
  let s = q.x * e.y - q.y * e.x;
  q = select(q, -q, s < 0.);
  var v: vec2<f32> = q - vec2<f32>(wi, 0.);
  v = v - e * clamp(dot(v, e) / dot(e, e), -1., 1.);
  d = min(d, vec2<f32>(dot(v, v), wi * he - abs(s)));
  return sqrt(d.x) * sign(-d.y);
}
/*
<img src="https://i.imgur.com/ilKWgT0.png" width="150px">

#### Equilateral Triangle - exact
*/
fn sdEquilateralTriangle(p: vec2<f32>) -> f32 {
  let k = sqrt(3.);
  var q: vec2<f32> = vec2<f32>(abs(p.x) - 1.0, p.y + 1. / k);
  if (q.x + k * q.y > 0.) { q = vec2<f32>(q.x - k * q.y, -k * q.x - q.y) / 2.; }
  q.x = q.x - clamp(q.x, -2., 0.);
  return -length(q) * sign(q.y);
}
/*
<img src="https://i.imgur.com/7u1IbCE.png" width="150px">

#### Isosceles Triangle - exact
*/
fn sdTriangleIsosceles(p: vec2<f32>, c: vec2<f32>) -> f32 {
  let q = vec2<f32>(abs(p.x), p.y);
  let a = q - c * clamp(dot(q, c) / dot(c, c), 0., 1.);
  let b = q - c * vec2<f32>(clamp(q.x / c.x, 0., 1.), 1.);
  let s = -sign(c.y);
  let d = min(vec2<f32>(dot(a, a), s * (q.x * c.y - q.y * c.x)), vec2<f32>(dot(b, b), s * (q.y - c.y)));
  return -sqrt(d.x) * sign(d.y);
}
/*
<img src="https://i.imgur.com/j56DPW1.png" width="150px">

#### Triangle - exact
*/
fn sdTriangle(p: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>) -> f32 {
  let e0 = p1 - p0; let e1 = p2 - p1; let e2 = p0 - p2;
  let v0 = p - p0; let v1 = p - p1; let v2 = p - p2;
  let pq0 = v0 - e0 * clamp(dot(v0, e0) / dot(e0, e0), 0., 1.);
  let pq1 = v1 - e1 * clamp(dot(v1, e1) / dot(e1, e1), 0., 1.);
  let pq2 = v2 - e2 * clamp(dot(v2, e2) / dot(e2, e2), 0., 1.);
  let s = sign(e0.x * e2.y - e0.y * e2.x);
  let d = min(min(vec2<f32>(dot(pq0, pq0), s * (v0.x * e0.y - v0.y * e0.x)),
                  vec2<f32>(dot(pq1, pq1), s * (v1.x * e1.y - v1.y * e1.x))),
                  vec2<f32>(dot(pq2, pq2), s * (v2.x * e2.y - v2.y * e2.x)));
  return -sqrt(d.x) * sign(d.y);
}
/*
<img src="https://i.imgur.com/zI2YOy7.png" width="150px">

#### Uneven Capsule - exact
*/
fn sdUnevenCapsule(p: vec2<f32>, r1: f32, r2: f32, h: f32) -> f32 {
  let q = vec2<f32>(abs(p.x), p.y);
  let b = (r1 - r2) / h;
  let a = sqrt(1. - b * b);
  let k = dot(q, vec2<f32>(-b, a));
  if (k < 0.) { return length(q) - r1; }
  if (k > a * h) { return length(q - vec2<f32>(0., h)) - r2; }
  return dot(q, vec2<f32>(a, b)) - r1;
}
/*
<img src="https://i.imgur.com/A48Bb32.png" width="150px">

#### Regular Pentagon - exact
*/
fn sdPentagon(p: vec2<f32>, r: f32) -> f32 {
  let k = vec3<f32>(0.809016994, 0.587785252, 0.726542528);
  var q: vec2<f32> = vec2<f32>(abs(p.x), p.y);
  q = q - 2. * min(dot(vec2<f32>(-k.x, k.y), q), 0.) * vec2<f32>(-k.x, k.y);
  q = q - 2. * min(dot(vec2<f32>(k.x, k.y), q), 0.) * vec2<f32>(k.x, k.y);
  q = q - vec2<f32>(clamp(q.x, -r * k.z, r * k.z), r);
  return length(q) * sign(q.y);
}
/*
<img src="https://i.imgur.com/8j0cNOU.png" width="150px">

#### Regular Hexagon - exact
*/
fn sdHexagon(p: vec2<f32>, r: f32) -> f32 {
  let k = vec3<f32>(-0.866025404, 0.5, 0.577350269);
  var q: vec2<f32> = abs(p);
  q = q - 2. * min(dot(k.xy, q), 0.) * k.xy;
  q = q - vec2<f32>(clamp(q.x, -k.z * r, k.z * r), r);
  return length(q) * sign(q.y);
}
/*
<img src="https://i.imgur.com/qKxPklx.png" width="150px">

#### Regular Octogon - exact
*/
fn sdOctogon(p: vec2<f32>, r: f32) -> f32 {
  let k = vec3<f32>(-0.9238795325, 0.3826834323, 0.4142135623);
  var q: vec2<f32> = abs(p);
  q = q - 2. * min(dot(vec2<f32>(k.x, k.y), q), 0.) * vec2<f32>(k.x, k.y);
  q = q - 2. * min(dot(vec2<f32>(-k.x, k.y), q), 0.) * vec2<f32>(-k.x, k.y);
  q = q - vec2<f32>(clamp(q.x, -k.z * r, k.z * r), r);
  return length(q) * sign(q.y);
}
/*
<img src="https://i.imgur.com/dkmvKGT.png" width="150px">

#### Hexagram - exact
*/
fn sdHexagram(p: vec2<f32>, r: f32) -> f32 {
  let k = vec4<f32>(-0.5, 0.8660254038, 0.5773502692, 1.7320508076);
  var q: vec2<f32> = abs(p);
  q = q - 2. * min(dot(k.xy, q), 0.) * k.xy;
  q = q - 2. * min(dot(k.yx, q), 0.) * k.yx;
  q = q - vec2<f32>(clamp(q.x, r * k.z, r * k.w), r);
  return length(q) * sign(q.y);
}
/*
<img src="https://i.imgur.com/tqQANNs.png" width="150px">

#### Star 5 - exact
*/
fn sdStar5(p: vec2<f32>, r: f32, rf: f32) -> f32 {
  let k1 = vec2<f32>(0.809016994375, -0.587785252292);
  let k2 = vec2<f32>(-k1.x, k1.y);
  var q: vec2<f32> = vec2<f32>(abs(p.x), p.y);
  q = q - 2. * max(dot(k1, q), 0.) * k1;
  q = q - 2. * max(dot(k2, q), 0.) * k2;
  q.x = abs(q.x);
  q.y = q.y - r;
  let ba = rf * vec2<f32>(-k1.y, k1.x) - vec2<f32>(0., 1.);
  let h = clamp(dot(q, ba) / dot(ba, ba), 0., r);
  return length(q - ba * h) * sign(q.y * ba.x - q.x * ba.y);
}
/*
<img src="https://i.imgur.com/fpxiUq9.png" width="150px">

#### Regular Star - exact
*/
fn sdStar(p: vec2<f32>, r: f32, n: u32, m: f32) ->f32 {
  let an = 3.141593 / f32(n);
  let en = 3.141593 / m;
  let acs = vec2<f32>(cos(an), sin(an));
  let ecs = vec2<f32>(cos(en), sin(en));
  let bn = (atan2(abs(p.x), p.y) % (2. * an)) - an;
  var q: vec2<f32> = length(p) * vec2<f32>(cos(bn), abs(sin(bn)));
  q = q - r * acs;
  q = q + ecs * clamp(-dot(q, ecs), 0., r * acs.y / ecs.y);
  return length(q) * sign(q.x);
}
/*
<img src="https://i.imgur.com/J9OZ1Tq.png" width="150px">

#### Pie - exact
*/
fn sdPie(p: vec2<f32>, sc: vec2<f32>, r: f32) -> f32 {
  let q = vec2<f32>(abs(p.x), p.y);
  let l = length(q) - r;
  let m = length(q - sc * clamp(dot(q, sc), 0., r));
  return max(l, m * sign(sc.y * q.x - sc.x * q.y));
}
/*
<img src="https://i.imgur.com/FjUdUAo.png" width="150px">

#### Arc - exact
*/
fn sdArc(p: vec2<f32>, sc1: vec2<f32>, sc2: vec2<f32>, r1: f32, r2: f32) -> f32 {
  var q: vec2<f32> = p * mat2x2<f32>(vec2<f32>(sc1.x, sc1.y), vec2<f32>(-sc1.y, sc1.x));
  q.x = abs(q.x);
  let k = select(length(q), dot(q, sc2), sc2.y * q.x > sc2.x * q.y);
  return sqrt(dot(q, q) + r1 * r1 - 2. * r1 * k) - r2;
}
/*
<img src="https://i.imgur.com/QRV4PPl.png" width="150px">

#### Horseshoe - exact
*/
fn sdHorseshoe(p: vec2<f32>, sc: vec2<f32>, r: f32, l: f32, w: f32) -> f32 {
  var q: vec2<f32> = vec2<f32>(abs(p.x), p.y);
  let m = length(p);
  q = q * mat2x2<f32>(vec2<f32>(-sc.y, sc.x), vec2<f32>(sc.x, sc.y));
  q = vec2<f32>(select(m * sign(-sc.y), q.x, q.y > 0.0 || q.x > 0.), select(m, q.y, q.x > 0.));
  q = vec2<f32>(q.x, abs(q.y - r)) - vec2<f32>(l, w);
  return length(max(q, vec2<f32>(0.))) + min(0., max(q.x, q.y));
}
/*
<img src="https://i.imgur.com/xixCcuP.png" width="150px">

#### Vesica - exact
*/
fn sdVesica(p: vec2<f32>, r: f32, d: f32) -> f32 {
  let q = abs(p);
  let b = sqrt(r * r - d * d);
  let cond = (q.y -b) * d > q.x * b;
  return select(length(q - vec2<f32>(-d, 0.))-r, length(q - vec2<f32>(0., b)), cond);
}
/*
<img src="https://i.imgur.com/QAyvVo4.png" width="150px">

#### Moon - exact
*/
fn sdMoon(p: vec2<f32>, d: f32, ra: f32, rb: f32) -> f32 {
  let q = vec2<f32>(p.x, abs(p.y));
  let a = (ra * ra - rb * rb + d * d) / (2. * d);
  let b = sqrt(max(ra * ra - a * a, 0.));
  if (d * (q.x * b - q.y * a) > d * d * max(b - q.y, 0.)) { return length(q-vec2<f32>(a, b)); }
  return max((length(q) - ra), -(length(q - vec2<f32>(d, 0.)) - rb));
}
/*
<img src="https://i.imgur.com/dUjwWV8.png" width="150px">

#### Rounded Cross - exact
*/
fn sdRoundedCross(p: vec2<f32>, h: f32) -> f32 {
  let k = 0.5 * (h + 1. / h);
  let q = abs(p);
  let v1 = q - vec2<f32>(1., k);
  let v2 = q - vec2<f32>(0., h);
  let v3 = q - vec2<f32>(1., 0.);
  let d1 = k - sqrt(dot(v1, v1));
  let d2 = sqrt(min(dot(v2, v2), dot(v3, v3)));
  return select(d2, d1, q.x < 1. && q.y < q.x * (k - h) + h);
}
/*
<img src="https://i.imgur.com/lraPt7s.png" width="150px">

#### Egg - exact
*/
fn sdEgg(p: vec2<f32>, ra: f32, rb: f32) -> f32 {
  let k = sqrt(3.);
  let q = vec2<f32>(abs(p.x), p.y);
  let r = ra - rb;
  let d1 = length(q) - r;
  let d2 = length(vec2<f32>(q.x,  q.y - k * r));
  let d3 = length(vec2<f32>(q.x + r, q.y)) - 2. * r;
  return select(select(d3, d2, k * (q.x + r) < q.y), d1, q.y < 0.) - rb;
}
/*
<img src="https://i.imgur.com/fjTSRvN.png" width="150px">

#### Heart - exact
*/
fn sdHeart(p: vec2<f32>) -> f32 {
  let q = vec2<f32>(abs(p.x), p.y);
  let w = q - vec2<f32>(0.25, 0.75);
  if (q.x + q.y > 1.0) { return sqrt(dot(w, w)) - sqrt(2.) / 4.; }
  let u = q - vec2<f32>(0., 1.);
  let v = q - 0.5 * max(q.x + q.y, 0.);
  return sqrt(min(dot(u, u), dot(v, v))) * sign(q.x - q.y);
}
/*
<img src="https://i.imgur.com/sYNlwJp.png" width="150px">

#### Cross - exact exterior, bound interior
*/
fn sdCross(p: vec2<f32>, b: vec2<f32>) -> f32 {
  var q: vec2<f32> = abs(p);
  q = select(q.xy, q.yx, q.y > q.x);
  let t = q - b;
  let k = max(t.y, t.x);
  let w = select(vec2<f32>(b.y - q.x, -k), t, k > 0.);
  return sign(k) * length(max(w, vec2<f32>(0.)));
}
/*
<img src="https://i.imgur.com/kCGLtYy.png" width="150px">

#### Rounded X - exact
*/
fn sdRoundedX(p: vec2<f32>, w: f32, r: f32) -> f32 {
  let q = abs(p);
  return length(q - min(q.x + q.y, w) * 0.5) - r;
}
/*
<img src="https://i.imgur.com/B6c7Ueg.png" width="150px">

#### Polygon - exact
*/
// fn sdPolygon(p: vec2<f32>, v: ptr<function, array<vec2<f32>, 5>>) -> f32 {
// 	let N: i32 = 5;
//   let c = *v;
//   var d = dot(p - c[0], p - c[0]);
//   var s: f32 = 1.;
//   for (var i: i32 = 0; i < N; i = i + 1) {
//     let j = (i + 1) % N;
//     let e = c[i] - c[j];
//     let w = p - c[j];
//     let b = w - e * clamp(dot(w, e) / dot(e, e), 0., 1.);
//     d = min(d, dot(b, b));
//     let c = vec3<bool>(p.y >= c[j].y, p.y < c[i].y, e.x * w.y > e.y * w.x);
//     if (all(c) || all(!c)) { s = -s; };
//   }
//   return s * sqrt(d);
// }
/*
<img src="https://i.imgur.com/BSh9Z8d.png" width="150px">

#### Ellipse - exact
*/
fn sdEllipse(p: vec2<f32>, ab: vec2<f32>) -> f32 {
  var q: vec2<f32> = abs(p);
  var e: vec2<f32> = ab;
  if (q.x > q.y) {
    q = q.yx;
    e = ab.yx;
  }
  let l = e.y * e.y - e.x * e.x;
  let m = e.x * q.x / l;
  let m2 = m * m;
  let n = e.y * q.y / l;
  let n2 = n * n;
  let c = (m2 + n2 - 1.) / 3.;
  let c3 = c * c * c;
  let b = c3 + m2 * n2 * 2.;
  let d = c3 + m2 * n2;
  let g = m + m * n2;
  var co: f32;
  if (d < 0.) {
    let h = acos(b / c3) / 3.0;
    let s = cos(h);
    let t = sin(h) * sqrt(3.);
    let rx = sqrt(-c * (s + t + 2.0) + m2);
    let ry = sqrt(-c * (s - t + 2.0) + m2);
    co = (ry + sign(l) * rx + abs(g) / (rx * ry) - m) / 2.;
  } else {
    let h = 2. * m * n * sqrt(d);
    let s = sign(b + h) * pow(abs(b + h), 1. / 3.);
    let u = sign(b - h) * pow(abs(b - h), 1. / 3.);
    let rx = -s - u - c * 4. + 2. * m2;
    let ry = (s - u) * sqrt(3.);
    let rm = sqrt(rx * rx + ry * ry);
    co = (ry / sqrt(rm - rx) + 2. * g / rm - m) / 2.;
  }
  let r = e * vec2<f32>(co, sqrt(1.0-co*co));
  return length(r - q) * sign(q.y - r.y);
}
/*
<img src="https://i.imgur.com/eS9IKh3.png" width="150px">

#### Parabola - exact
*/
fn sdParabola(pos: vec2<f32>, k: f32) -> f32 {
  let p = vec2<f32>(abs(pos.x), pos.y);
  let ik = 1. / k;
  let u = ik * (p.y - 0.5 * ik) / 3.;
  let v = 0.25 * ik * ik * p.x;
  let h = v * v - u * u * u;
  let r = sqrt(abs(h));
  let x = select(2. * cos(atan2(r, v) / 3.) * sqrt(u),
    pow(v + r, 1. / 3.) - pow(abs(v - r), 1. / 3.) * sign(r - v),
    h > 0.0);
  return length(p - vec2<f32>(x, k * x * x)) * sign(p.x - x);
}
/*
<img src="https://i.imgur.com/YRTCvHG.png" width="150px">

#### Parabola Segment - exact
*/
fn sdParabolaSegment(pos: vec2<f32>, wi: f32, he: f32) -> f32 {
  let p = vec2<f32>(abs(pos.x), pos.y);
  let ik = wi * wi / he;
  let u = ik * (he - p.y - 0.5 * ik) / 3.;
  let v = p.x * ik * ik * 0.25;
  let h = v * v - u * u * u;
  let r = sqrt(abs(h));
  var x: f32 = select(2. * cos(atan(r / v) / 3.) * sqrt(u),
    pow(v + r, 1. / 3.) - pow(abs(v - r), 1. / 3.) * sign(r - v),
    h > 0.0);
  x = min(x, wi);
  return length(p - vec2<f32>(x, he - x * x / ik)) * sign(ik * (p.y - he) + p.x * p.x);
}
/*
<img src="https://i.imgur.com/XtQaOJ5.png" width="150px">

#### Quadratic Bezier - exact
*/
fn sdBezier(p: vec2<f32>, A: vec2<f32>, B: vec2<f32>, C: vec2<f32>) -> vec2<f32> {
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
<img src="https://i.imgur.com/3mXMdZA.png" width="150px">

#### Bobbly Cross - exact
*/
fn sdBlobbyCross(pos: vec2<f32>, he: f32) -> f32 {
  var p: vec2<f32> = abs(pos);
  p = vec2<f32>(abs(p.x - p.y), 1. - p.x - p.y) / sqrt(2.);

  let u = (he - p.y - 0.25 / he) / (6. * he);
  let v = p.x / (he * he * 16.);
  let h = v * v - u * u * u;

  var x: f32; var y: f32;
  if (h > 0.) {
    let r = sqrt(h);
    x = pow(v + r, 1. / 3.) - pow(abs(v - r), 1. / 3.) * sign(r - v);
  } else {
    let r = sqrt(u);
    x = 2. * r * cos(acos(v / (u * r)) / 3.);
  }
  x = min(x, sqrt(2.) / 2.);

  let z = vec2<f32>(x, he * (1. - 2. * x * x)) - p;
  return length(z) * sign(z.y);
}
/*
<img src="https://i.imgur.com/1YDqsX7.png" width="150px">
*/