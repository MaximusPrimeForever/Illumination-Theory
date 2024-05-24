
// == Constants ==
const MAX_T: f32 = 3.4028235e+38;
// == /Constants ==

alias TriangleVertices = array<vec2f, 6>;
var<private> vertices: TriangleVertices = TriangleVertices(
  vec2f(-1.0,  1.0),
  vec2f(-1.0, -1.0),
  vec2f( 1.0,  1.0),
  vec2f( 1.0,  1.0),
  vec2f(-1.0, -1.0),
  vec2f( 1.0, -1.0),
);

struct Uniforms {
  width: u32,
  height: u32,
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

// === Ray ===
struct Ray {
  origin: vec3f,
  direction: vec3f,
}

fn ray_at(ray: Ray, t: f32) -> vec3f {
  return ray.origin + t * ray.direction;
}
// === /Ray ===

// === Intersection ===
struct Intersection {
  normal: vec3f,
  t: f32,
}

fn no_intersection() -> Intersection {
  return Intersection(vec3(0.), -1.);
}

const INF_intersection = Intersection(vec3(0.), MAX_T);
// === /Intersection ===

// === Sphere ===
struct Sphere {
  center: vec3f,
  radius: f32,
}

const OBJECT_COUNT: u32 = 2;
alias Scene = array<Sphere, OBJECT_COUNT>;
var<private> scene: Scene = Scene(
  Sphere(vec3(0., 0., -1.), 0.5),
  Sphere(vec3(0., -100.5, -1.), 100.),
);

fn intersect_sphere(ray: Ray, sphere: Sphere) -> Intersection {
  let v = ray.origin - sphere.center;
  let a = dot(ray.direction, ray.direction);
  let b = dot(v, ray.direction);
  let c = dot(v, v) - sphere.radius * sphere.radius;

  let d = b * b - a * c;
  if d < 0. {
    return no_intersection();
  }

  let sqrt_d = sqrt(d);
  let recip_a = 1. / a;
  let mb = -b;
  let t1 = (mb - sqrt_d) * recip_a;
  let t2 = (mb + sqrt_d) * recip_a;
  // apparently, if `cond` is true then t1 is returned, and not t2
  let t = select(t2, t1, t1 > 0.);
  if t <= 0. {
    return no_intersection();
  }

  let p = ray_at(ray, t);
  let normal = (p - sphere.center) / sphere.radius;
  return Intersection(normal, t);
}
// === /Sphere ===

fn sky_color(ray: Ray) -> vec3f {
  let t = 0.5 * (normalize(ray.direction).y + 1.);
  return (1. - t) * vec3(1., 0.5, 0.3) + t * vec3(0.3, 0.5, 1.);
}

// === Vertex Shader ===

@vertex fn display_vs(@builtin(vertex_index) vid: u32) -> @builtin(position) vec4f {
  return vec4f(vertices[vid], 0.0, 1.0);
}
// === /Vertex Shader ===

// === Fragment Shader ===
@fragment fn display_fs(@builtin(position) pos: vec4f) -> @location(0) vec4f {
  let origin = vec3(0.);
  let focus_distance = 1.;
  let aspect_ratio = f32(uniforms.width) / f32(uniforms.height);

  // Normalize the viewport coordinates.
  var uv = pos.xy / vec2f(f32(uniforms.width - 1u), f32(uniforms.height - 1u));

  // Map `uv` from y-down (normalized) viewport coordinates to camera coordinates.
  uv = (2. * uv - vec2(1.)) * vec2(aspect_ratio, -1.);

  let direction = vec3(uv, -focus_distance);
  let ray = Ray(origin, direction);
  
  var closest_intersection: Intersection = INF_intersection;
  for (var i: u32 = 0u; i < OBJECT_COUNT; i = i + 1u) {
    let sphere = scene[i];
    let intersection = intersect_sphere(ray, sphere);
    if intersection.t > 0. && intersection.t < closest_intersection.t {
        closest_intersection = intersection;
      }
  }
  if closest_intersection.t < INF_intersection.t {
    return vec4(closest_intersection.normal, 0.);
  }

  return vec4(sky_color(ray), 1.);
}