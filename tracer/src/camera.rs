use std::sync::Arc;

use rand::random;

use crate::ray::Ray;
use crate::world::World;
use crate::interval::Interval;
use crate::vec3::{Point3, Vec3, Color};
use crate::geometry::random_in_unit_disk;

use crate::color::sky_color;
use crate::color::COLOR_BLACK;

pub struct Camera {
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_horizontal: Vec3,
    pixel_delta_vertical: Vec3,
    basis_right: Vec3,
    basis_up: Vec3,
    basis_view: Vec3,
    defocus_disk_horizontal: Vec3,  // Defocus disk horizontal radius
    defocus_disk_vertical: Vec3,  // Defocus disk vertical radius

    // Optics
    pub vfov: f64,
    pub defocus_angle: f64,  // Variation angle of rays through each pixel
    pub focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus

    // Positions
    pub look_from: Point3,  // Point camera is looking from
    pub look_at: Point3,    // Point camera is looking at
    pub vup: Vec3,          // Camera-relative "up" direction

    // Renderer
    pub image_width: usize,
    pub image_height: usize,

    // Aux
    pub is_initialized: bool
}

pub const T_MIN_TOLERANCE: f64 = 0.001;


impl Default for Camera {
    fn default() -> Self {
        Camera { 
            center: Point3::zero(),
            pixel00_loc: Point3::default(),
            pixel_delta_horizontal: Vec3::default(),
            pixel_delta_vertical: Vec3::default(),
            basis_right: Vec3::default(),
            basis_up: Vec3::default(),
            basis_view: Vec3::default(),
            defocus_disk_horizontal: Vec3::default(),
            defocus_disk_vertical: Vec3::default(),
            vfov: 60.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
            look_from: Point3::new(0.0, 0.0, -1.0),
            look_at: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            image_width: 640,
            image_height: 360,
            is_initialized: false
        }
    }
}

impl Camera {
    /// The fact this method has to be called manually is pretty bad
    pub fn initialize(&mut self) {
        self.center = self.look_from;

        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (
            self.image_width as f64 / self.image_height as f64
        );

        self.basis_view = (self.look_from - self.look_at).unit();
        self.basis_right = (self.vup.cross(self.basis_view)).unit();
        self.basis_up = self.basis_view.cross(self.basis_right);

        let viewport_horizontal = viewport_width * self.basis_right;
        let viewport_vertical = viewport_height * (-self.basis_up);

        self.pixel_delta_horizontal = viewport_horizontal / self.image_width as f64;
        self.pixel_delta_vertical = viewport_vertical / self.image_height as f64;

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_horizontal = self.basis_right * defocus_radius;
        self.defocus_disk_vertical = self.basis_up * defocus_radius;

        let viewport_upper_left =
            self.center
            - (self.focus_dist * self.basis_view)
            - viewport_horizontal / 2.0
            - viewport_vertical / 2.0;
        
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_horizontal +  self.pixel_delta_vertical);
        self.is_initialized = true;
    }

    /// Return a ray's color on the viewport
    pub fn render_ray(&self,
                      row: f64,
                      col: f64,
                      world: &Arc<World>,
                      trace_depth: usize) -> Color {
        if !self.is_initialized {
            panic!("Camera must be initialized before rendering.");
        }
        let ray = self.generate_ray(row, col);
        self.ray_color(ray, world, trace_depth, false)
    }

    /// Get a randomly-sampled camera ray for the pixel at location i,j, originating from
    /// the camera defocus disk.
    fn generate_ray(&self, row: f64, col: f64) -> Ray {
        let pixel_center = self.pixel00_loc + (row * self.pixel_delta_vertical) + (col * self.pixel_delta_horizontal);
        let pixel_sample = pixel_center + self.sample_pixel_square();

        let ray_origin = self.sample_defocus_disk();
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction, rand::random::<f64>())
    }

    /// Return a random point in the square surrounding a pixel at the origin.
    fn sample_pixel_square(&self) -> Vec3 {
        let px = -0.5 + random::<f64>();
        let py = -0.5 + random::<f64>();

        (px * self.pixel_delta_horizontal) + (py * self.pixel_delta_vertical)
    }

    /// Return a random point in the camera defocus disk.
    fn sample_defocus_disk(&self) -> Point3 {
        let p = random_in_unit_disk();

        self.center + (p[0] * self.defocus_disk_horizontal) + (p[1] * self.defocus_disk_vertical)
    }

    /// Render the color of a single ray shot into the world.
    fn ray_color(&self, ray: Ray, world: &Arc<World>, trace_depth: usize, has_bounced: bool) -> Color {
        if trace_depth <= 0 { 
            // Once depth runs out, generate rays to all lights in the world
            // and for each ray check if it's a shadow ray or light ray
            // return COLOR_BLACK;
            return world.hit_lights(ray.origin, T_MIN_TOLERANCE);
        }
        
        match world.hit_object(ray, Interval::new(T_MIN_TOLERANCE, f64::INFINITY)) {
            Some(rec) => {
                match rec.material.scatter(&ray, &rec) {
                    Some((attenuation, scattered)) => {
                        attenuation * self.ray_color(
                            scattered, 
                            world, 
                            trace_depth - 1,
                            true)
                    }
                    None => { COLOR_BLACK }
                }
            }
            None => {
                // Ray did not hit any object
                let sky_color = sky_color(ray);
    
                // return the sky color if the ray hasn't bounced around
                // this results in a nice color for the sky
                if !has_bounced {
                    return sky_color;
                }

                if world.lights.len() == 0 {
                    return sky_color;
                }
                
                // Compute light value based on sky and nearby lights
                // very crude and lame
                let lights_color = world.hit_lights(ray.origin, T_MIN_TOLERANCE);
                return lights_color * 0.8 + sky_color * 0.2;
            }
        }
    }
}