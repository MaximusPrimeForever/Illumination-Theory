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

struct Ray {
  origin: vec3f,
  direction: vec3f,
}

struct Sphere {
  center: vec3f,
  radius: f32,
}

const MAX_T: f32 = 3.4028235e+38;
const OBJECT_COUNT: u32 = 2;
alias Scene = array<Sphere, OBJECT_COUNT>;
var<private> scene: Scene = Scene(
  Sphere(vec3(0., 0., -1.), 0.5),
  Sphere(vec3(0., -100.5, -1.), 100.),
);

fn intersect_sphere(ray: Ray, sphere: Sphere) -> f32 {
  let v = ray.origin - sphere.center;
  let a = dot(ray.direction, ray.direction);
  let b = dot(v, ray.direction);
  let c = dot(v, v) - sphere.radius * sphere.radius;

  let d = b * b - a * c;
  if d < 0. {
    return -1.;
  }

  let sqrt_d = sqrt(d);
  let recip_a = 1. / a;
  let mb = -b;
  let t = (mb - sqrt_d) * recip_a;
  if t > 0. {
    return t;
  }
  return (mb + sqrt_d) * recip_a;
}

fn sky_color(ray: Ray) -> vec3f {
  let t = 0.5 * (normalize(ray.direction).y + 1.);
  return (1. - t) * vec3(1., 0.5, 0.3) + t * vec3(0.3, 0.5, 1.);
}

@vertex fn display_vs(@builtin(vertex_index) vid: u32) -> @builtin(position) vec4f {
  return vec4f(vertices[vid], 0.0, 1.0);
}

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
  
  var closest_t: f32 = MAX_T;
  for (var i: u32 = 0u; i < OBJECT_COUNT; i = i + 1u) {
    let sphere = scene[i];
    let intersection = intersect_sphere(ray, sphere);
    if intersection > 0. {
      if intersection < closest_t {
        closest_t = intersection;
      }
    }
  }
  if closest_t < MAX_T {
    let color = vec3(1., 0.5, 0.) * (1. - closest_t);
    return vec4(color, 0.);
  }

  return vec4(sky_color(ray), 1.);
}