#![allow(dead_code)]
/// This module contains mostly scene descriptions, and some util functions.

use std::sync::Arc;

use rand::random;
use crate::geometry::ConstantMedium;
use crate::graphics::Camera;
use crate::math::{
    utils::random_f64_in_range,
    vec3::{Color, Point3, Vec3}
};
use crate::geometry::{
    Quad, 
    Sphere,
    box_new, 
    RotateY, 
    Translate,
    new_sphereflake_upright, 
    hittable::{HittableSync, HittableComposite}, 
};

use crate::graphics::{
    bvh::BVH,
    light::DiffuseLight,
    material::{Lambertian, Metal, Dielectric, MaterialSync},
    texture::{SolidColorTexture, CheckerTexture, ImageTexture, NoiseTexture},
};

use crate::rendering::{
    render::render_scene, 
    color::{COLOR_SKY_BLUE, COLOR_BLACK, COLOR_WHITE}
};

fn generate_default_plane(plane_size: f64, color: Option<Color>) -> Quad {
    let plane_color = color.unwrap_or(Color::new(0.8, 0.8, 0.8));
    let plane_material = Arc::new(Lambertian::new(plane_color));

    Quad::new(
        Point3::new(-(plane_size / 2.0), 0.0, -(plane_size / 2.0)),
        Vec3::new(plane_size, 0.0, 0.0),
        Vec3::new(0.0, 0.0, plane_size),
        plane_material
    )
}

pub fn test_scene() {
    // Camera    
    let mut cam = Camera::default();
    cam.look_from = Point3::new(0.8, 1.0, 2.0);
    cam.look_at = Point3::new(0.8, 0.0, -1.0);
    cam.background = Color::new(0.7, 0.8, 1.1);

    let mut objects: Vec<Arc<HittableSync>> = Vec::new();

    // plane
    objects.push(Arc::new(generate_default_plane(30.0, Some(Color::new(0.5, 0.5, 0.5)))));
    
    // spheres
    let sphere_radius = 0.3;
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, sphere_radius, 0.0),
        sphere_radius,
        Arc::new(Dielectric{ir: 1.5})
    )));
    
    objects.push(Arc::new(Sphere::new(
        Point3::new(-sphere_radius*2.0 - 0.1, sphere_radius, 0.0),
        sphere_radius,
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)))
    )));

    objects.push(Arc::new(Sphere::new(
        Point3::new(sphere_radius*2.0 + 0.1, sphere_radius, 0.0),
        sphere_radius,
        Arc::new(Metal{albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0})
    )));

    // quads
    objects.push(Arc::new(Quad::new(
        Point3::new(1.1, 0.0, 0.0),
        Vec3::new(0.3, 0.0, 0.0),
        Vec3::new(0.0, 0.6, 0.0),
        Arc::new(Lambertian::new(Color::new(0.9, 0.1, 0.1)))
    )));
    
    // fuzzy mirror
    objects.push(Arc::new(Quad::new(
        Point3::new(-3.0, 0.0, -0.8),
        Vec3::new(8.0, 0.0, 0.0),
        Vec3::new(0.0, 1.2, 0.0),
        Arc::new(Metal{albedo: Color::new(0.8, 0.7, 0.6), fuzz: 0.05})
    )));

    objects.push(Arc::new(Quad::new(
        Point3::new(1.5, 0.0, 0.0),
        Vec3::new(0.3, 0.0, 0.0),
        Vec3::new(0.0, 0.6, 0.0),
        Arc::new(Metal{albedo: Color::new(0.1, 0.1, 0.9), fuzz: 0.0})
    )));

    // lights
    
    let world = HittableComposite::new_from_objects(objects);

    // Must be called!
    cam.initialize();

    // Render
    let image_canvas = render_scene(
        0,
        Arc::new(world),
        Arc::new(cam),
        100,
        10
    );
    
    // Output to file
    image_canvas.save_png("test_scene.png")
}

pub fn one_weekend_endgame(cam: &mut Camera, grid_size: i32) -> Arc<HittableSync> {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    objects.push(Arc::new(
        Sphere::new(
            Point3::new(0.0, -1000.0, 0.0), 
            1000.0, 
            ground_material
        )
    ));

    let some_point = Point3::new(4.0, 0.2, 0.0);

    for a in -grid_size..grid_size {
        for b in -grid_size..grid_size {
            let choose_material = random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>()
            );

            if (center - some_point).length() > 0.9 {
                let sphere_material: Arc<MaterialSync>;
                let sphere: Sphere;

                // pick material
                if choose_material < 0.6 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));

                } else if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let fuzz = random_f64_in_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal{albedo, fuzz});

                } else {
                    sphere_material = Arc::new(Dielectric{ir: 1.5});
                }
                sphere = Sphere::new(center, 0.2, sphere_material);
                objects.push(Arc::new(sphere));
            }
        }
    }

    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric{ir: 1.5})
    )));
    
    objects.push(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)))
    )));

    objects.push(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal{albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0})
    )));

    // Camera
    cam.defocus_angle = 0.6;
    cam.focus_dist = 13.38;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 1.5, 0.0);
    cam.background = COLOR_SKY_BLUE;

    Arc::new(HittableComposite::new_from_objects(objects))
}

pub fn cool_effects(sphere_count: u32, distance: f64) -> HittableComposite {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();

    // ground
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)))
    )));

    // big shiny ball
    let shiny = Metal{
        albedo: Color::new(1.4, 1.2, 1.0) * 0.5, 
        fuzz: 0.0
    };
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(shiny)
    )));

    let theta_rad = (2.0 * std::f64::consts::PI) / (sphere_count as f64);
    for i in 0..sphere_count {
        let x = distance * (theta_rad * i as f64).cos();
        let y = 0.2;
        let z = distance * (theta_rad * i as f64).sin();

        objects.push(Arc::new(Sphere::new(
            Point3::new(x, y, z),
            0.2, 
            Arc::new(Dielectric{ir: 1.5})
        )));
    }

    let theta_rad = (2.0 * std::f64::consts::PI) / (sphere_count as f64);
    for i in 0..sphere_count { 
        let x = (distance) * (theta_rad * 1.5 * i as f64).cos();
        let y = 0.2;
        let z = (distance) * (theta_rad * 1.5 * i as f64).sin();

        let albedo = Color::random() * Color::random() * 2.0;
        objects.push(Arc::new(Sphere::new(
            Point3::new(x, y, z),
            0.2, 
            Arc::new(Metal{albedo: albedo, fuzz: 0.0})
        )));
    }

    HittableComposite::new_from_objects(objects)
}

pub fn row_of_glass(sphere_count: u32, distance: f64) -> HittableComposite {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();

    // ground
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)))
    )));

    for i in 0..sphere_count { 
        // let albedo = Color::random() * Color::random() * 2.0;
        objects.push(Arc::new(Sphere::new(
            Point3::new((0.1 + distance) * i as f64, 0.2, 0.0),
            0.2, 
            Arc::new(Dielectric{ir: 1.5})
        )));
    }

    let final_row_z = sphere_count as f64 * (0.2 + distance);
    objects.push(Arc::new(Sphere::new(
        Point3::new(final_row_z, 0.2, 0.25),
        0.1,
        Arc::new(Lambertian::new(Color::new(2.0, 0.2, 0.2)))
    )));
    objects.push(Arc::new(Sphere::new(
        Point3::new(final_row_z, 0.2, 0.0),
        0.1,
        Arc::new(Lambertian::new(Color::new(0.2, 2.0, 0.2)))
    )));
    objects.push(Arc::new(Sphere::new(
        Point3::new(final_row_z, 0.2, -0.25),
        0.1,
        Arc::new(Lambertian::new(Color::new(0.2, 0.2, 2.0)))
    )));

    HittableComposite::new_from_objects(objects)
}

pub fn grid_of_glass(size: u32, distance: f64, radius: f64) -> HittableComposite {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();

    // ground
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)))
    )));

    let total_width = (size - 1) as f64 * (radius + distance);
    let starting_point = Point3::new(
        -total_width / 2.0,
        radius + 0.5, 
        -total_width / 2.0
    );

    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, total_width / 2.0 + 1.0, 0.0),
        0.1,
        Arc::new(Lambertian::new(Color::new(4.0, 0.2, 0.2)))
    )));

    for y in 0..size { 
        for z in 0..size {
            for x in 0..size {
                // let albedo = Color::random() * Color::random() * 2.0;
                objects.push(Arc::new(Sphere::new(
                    Point3::new(
                        starting_point.x() + x as f64 * (radius + distance),
                        starting_point.y() + y as f64 * (radius + distance),
                        starting_point.z() + z as f64 * (radius + distance)
                    ),
                    radius, 
                    Arc::new(Metal{albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0})
                )));
            }
        }
    }

    HittableComposite::new_from_objects(objects)
}

pub fn lit_world(cam: &mut Camera) -> HittableComposite {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();

    let noise_texture = Arc::new(Lambertian::new_texture(Arc::new(
        NoiseTexture::new(4.0)
    )));
    objects.push(Arc::new(
        Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0, 
            noise_texture.clone()
        )
    ));
    
    objects.push(Arc::new(
        Sphere::new(
            Point3::new(0.0, 2.0, 0.0),
            2.0,
            noise_texture
        )
    ));

    let diffuse_light = DiffuseLight::new_color(Color::new(4.0, 4.0, 4.0));
    objects.push(Arc::new(Quad::new(
        Point3::new(3, 1, -2),
        Vec3::new(2, 0, 0),
        Vec3::new(0, 2, 0),
        Arc::new(diffuse_light)
    )));

    cam.look_from = Point3::new(26, 3, 6);
    cam.look_at = Point3::new(0, 2, 0);
    cam.background = COLOR_BLACK;

    HittableComposite::new_from_objects(objects)
}

pub fn lit_world_textures(cam: &mut Camera) -> HittableComposite {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    objects.push(Arc::new(
        Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)
    ));

    let solid_texture = SolidColorTexture::new(
        Color::new(0.4, 0.2, 0.1)
    );
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 2.0),
        1.0,
        Arc::new(Lambertian::new_texture(Arc::new(solid_texture))))
    ));
    
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal{albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0})
    )));
    
    let checkered = CheckerTexture::new_color(
        0.3,
        Color::new(0.9, 0.9, 0.9),
        Color::new(0.2, 0.3, 0.1),
    );
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, -2.0),
        1.0,
        Arc::new(Lambertian::new_texture(Arc::new(checkered)))
    )));

    // Camera
    cam.defocus_angle = 0.6;
    cam.focus_dist = 13.38;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 1.5, 0.0);
    cam.background = COLOR_SKY_BLUE;
    
    HittableComposite::new_from_objects(objects)
}

pub fn two_checkered_spheres(cam: &mut Camera) -> HittableComposite {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();
    
    let checkered = CheckerTexture::new_color(
        0.3,
        Color::new(0.9, 0.9, 0.9),
        Color::new(0.2, 0.3, 0.1),
    );
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_texture(Arc::new(checkered.clone())))
    )));
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_texture(Arc::new(checkered)))
    )));

    // Camera
    cam.defocus_angle = 0.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.background = COLOR_SKY_BLUE;
    
    HittableComposite::new_from_objects(objects)
}

pub fn earth(cam: &mut Camera) -> HittableComposite {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();

    let earth_texture = ImageTexture::new("./resources/textures/earthmap.jpg");
    let earth_surface = Lambertian::new_texture(Arc::new(earth_texture));
    let globe = Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(earth_surface)
    );
    objects.push(Arc::new(globe));

    let ground_material = Arc::new(Metal::new(
        Color::new(0.5, 0.5, 0.5),
        0.05
    ));
    objects.push(Arc::new(
        Sphere::new(
            Point3::new(0.0, -1000.0, -1.0),
            1000.0, 
            ground_material
        )
    ));

    cam.look_from = Point3::new(20.0, 7.0, 0.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.background = COLOR_SKY_BLUE;


    HittableComposite::new_from_objects(objects)
}

pub fn marble_texture(cam: &mut Camera) -> HittableComposite {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();

    let noise_texture = Arc::new(Lambertian::new_texture(Arc::new(
        NoiseTexture::new(4.0)
    )));
    objects.push(Arc::new(
        Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0, 
            noise_texture.clone()
        )
    ));
    
    objects.push(Arc::new(
        Sphere::new(
            Point3::new(0.0, 2.0, 0.0),
            2.0,
            noise_texture
        )
    ));

    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.5, 0.0);
    cam.defocus_angle = 0.0;
    cam.background = COLOR_SKY_BLUE;

    HittableComposite::new_from_objects(objects)
}

pub fn quad_scene(cam: &mut Camera) -> HittableComposite {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();

    let red = Lambertian::new(Color::new(1.0, 0.2, 0.2));
    let green = Lambertian::new(Color::new(0.2, 1.0, 0.2));
    let blue = Lambertian::new(Color::new(0.2, 0.2, 1.0));
    let orange = Lambertian::new(Color::new(1.0, 0.5, 0.0));
    let teal = Lambertian::new(Color::new(0.2, 0.8, 0.8));

    objects.push(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(red)
    )));

    objects.push(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(green)
    )));

    objects.push(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(blue)
    )));

    objects.push(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        Arc::new(orange)
    )));

    objects.push(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Arc::new(teal)
    )));

    cam.look_from = Point3::new(0.0, 0.0, 9.0);
    cam.look_at = Point3::zero();
    cam.background = COLOR_SKY_BLUE;

    HittableComposite::new_from_objects(objects)
}

pub fn quad_shadow_test(cam: &mut Camera) -> HittableComposite {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();

    let white = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));

    objects.push(Arc::new(Quad::new(
        Point3::new(-10.5, 0.0, -10.0),
        Vec3::new(20.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 20.0),
        white
    )));

    objects.push(Arc::new(Quad::new(
        Point3::new(-2.0, 2.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 2.0),
        red
    )));

    objects.push(Arc::new(
        Sphere::new(
            Point3::new(2.0, 1.0, -2.0),
            1.0, 
            green
        )
    ));

    cam.look_from = Point3::new(0.0, 1.0, 3.0);
    cam.look_at = Point3::new(0.0, 1.0, 0.0);
    cam.background = COLOR_SKY_BLUE;

    HittableComposite::new_from_objects(objects)
}

pub fn cornell_box(cam: &mut Camera) -> Arc<HittableSync> {
    let mut world = HittableComposite::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let light = Arc::new(DiffuseLight::new_color(COLOR_WHITE * 15.0));

    let width = 100.0;
    let height = width;
    let length = width;
    let light_width = width * 0.2;
     let light_length = light_width;
    let bottom_left_corner = Point3::new(
        -(width / 2.0), -(height / 2.0), 0.0
    );
    
    // front wall
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone(),
        Vec3::new(width, 0.0, 0.0),
        Vec3::new(0.0, height, 0.0),
        white.clone()
    )));
    
    // floor
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone(),
        Vec3::new(width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, length),
        white.clone()
    )));

    // ceiling
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone() + Point3::new(0.0, height, 0.0),
        Vec3::new(width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, length),
        white.clone()
    )));

    // light
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone() + Point3::new((width - light_width) / 2.0, height - 1.0, (length - light_length) / 2.0),
        Vec3::new(light_width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, light_length),
        light
    )));

    // left wall
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone(),
        Vec3::new(0.0, 0.0, length),
        Vec3::new(0.0, height, 0.0),
        green
    )));

    // right wall
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone() + Point3::new(width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, length),
        Vec3::new(0.0, height, 0.0),
        red
    )));

    // boxes
    let tall_box_brc = bottom_left_corner + Point3::new(width / 6.0, 0.0, length / 4.0);
    let mut tall_box = box_new(
        Point3::zero(),
        Vec3::new(width / 3.5, height / 1.8, width / 3.5),
        white.clone()
    );
    tall_box = Arc::new(RotateY::new(tall_box, 15.0));
    tall_box = Arc::new(Translate::new(tall_box, tall_box_brc));
    world.add_hittable(tall_box);

    let cube_box_side = width / 3.5;
    let cube_box_brc = bottom_left_corner + Point3::new(width / 1.7, 0.0, length / 2.0);
    let mut cube_box = box_new(
        Point3::zero(),
        Vec3::new(cube_box_side, cube_box_side, cube_box_side),
        white.clone()
    );
    cube_box = Arc::new(RotateY::new(cube_box, -18.0));
    cube_box = Arc::new(Translate::new(cube_box, cube_box_brc));
    world.add_hittable(cube_box);

    cam.look_from = Point3::new(0.0, 0.0, length * 2.70);
    cam.look_at = Point3::zero();
    cam.vfov = 35.0;

    Arc::new(BVH::new(&mut world))
}

pub fn cornell_box_with_smokey_boxes(cam: &mut Camera) -> Arc<HittableSync> {
    let mut world = HittableComposite::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let light = Arc::new(DiffuseLight::new_color(COLOR_WHITE * 7.0));

    let width = 100.0;
    let height = width;
    let length = width;
    let light_width = width * 0.4;
     let light_length = light_width;
    let bottom_left_corner = Point3::new(
        -(width / 2.0), -(height / 2.0), 0.0
    );
    
    // front wall
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone(),
        Vec3::new(width, 0.0, 0.0),
        Vec3::new(0.0, height, 0.0),
        white.clone()
    )));
    
    // floor
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone(),
        Vec3::new(width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, length),
        white.clone()
    )));

    // ceiling
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone() + Point3::new(0.0, height, 0.0),
        Vec3::new(width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, length),
        white.clone()
    )));

    // light
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone() + Point3::new((width - light_width) / 2.0, height - 1.0, (length - light_length) / 2.0),
        Vec3::new(light_width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, light_length),
        light
    )));

    // left wall
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone(),
        Vec3::new(0.0, 0.0, length),
        Vec3::new(0.0, height, 0.0),
        green
    )));

    // right wall
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone() + Point3::new(width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, length),
        Vec3::new(0.0, height, 0.0),
        red
    )));

    // boxes
    let tall_box_brc = bottom_left_corner + Point3::new(width / 6.0, 0.0, length / 4.0);
    let mut tall_box = box_new(
        Point3::zero(),
        Vec3::new(width / 3.5, height / 1.8, width / 3.5),
        white.clone()
    );
    tall_box = Arc::new(RotateY::new(tall_box, 15.0));
    tall_box = Arc::new(Translate::new(tall_box, tall_box_brc));
    world.add_hittable(Arc::new(
        ConstantMedium::new_color(
            tall_box, 
            0.04, 
            Color::new(0.0, 0.0, 0.0)
    )));

    let cube_box_side = width / 3.5;
    let cube_box_brc = bottom_left_corner + Point3::new(width / 1.7, 0.0, length / 2.0);
    let mut cube_box = box_new(
        Point3::zero(),
        Vec3::new(cube_box_side, cube_box_side, cube_box_side),
        white.clone()
    );
    cube_box = Arc::new(RotateY::new(cube_box, -18.0));
    cube_box = Arc::new(Translate::new(cube_box, cube_box_brc));
    world.add_hittable(Arc::new(
        ConstantMedium::new_color(
            cube_box, 
            0.04, 
            Color::new(1.0, 1.0, 1.0)
    )));

    cam.look_from = Point3::new(0.0, 0.0, length * 2.70);
    cam.look_at = Point3::zero();
    cam.vfov = 35.0;

    Arc::new(BVH::new(&mut world))
}

pub fn cornell_box_dark_sphereflake(cam: &mut Camera) -> Arc<HittableSync> {
    let mut world = HittableComposite::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let light = Arc::new(DiffuseLight::new_color(COLOR_WHITE * 25.0));

    let width = 100.0;
    let height = width;
    let length = width;
    let light_width = width * 0.2;
     let light_length = light_width;
    let bottom_left_corner = Point3::new(
        -(width / 2.0), -(height / 2.0), 0.0
    );
    
    // front wall
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone(),
        Vec3::new(width, 0.0, 0.0),
        Vec3::new(0.0, height, 0.0),
        white.clone()
    )));
    
    // floor
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone(),
        Vec3::new(width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, length),
        white.clone()
    )));

    // ceiling
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone() + Point3::new(0.0, height, 0.0),
        Vec3::new(width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, length),
        white.clone()
    )));

    // light
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone() + Point3::new((width - light_width) / 2.0, height - 1.0, (length - light_length) / 2.0),
        Vec3::new(light_width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, light_length),
        light
    )));

    // left wall
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone(),
        Vec3::new(0.0, 0.0, length),
        Vec3::new(0.0, height, 0.0),
        green
    )));

    // right wall
    world.add_hittable(Arc::new(Quad::new(
        bottom_left_corner.clone() + Point3::new(width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, length),
        Vec3::new(0.0, height, 0.0),
        red
    )));

    // boxes
    let tall_box_brc = bottom_left_corner + Point3::new(width / 7.0, 0.0, length / 4.0);
    let mut tall_box = box_new(
        Point3::zero(),
        Vec3::new(width / 3.5, height / 1.8, width / 3.5),
        white
    );
    tall_box = Arc::new(RotateY::new(tall_box, 15.0));
    tall_box = Arc::new(Translate::new(tall_box, tall_box_brc));
    world.add_hittable(tall_box);


    let radius = width / 6.0;
    let mirror = Arc::new(Metal::new_mirror(Color::new(0.1, 0.1, 0.1)));
    let sphereflake = new_sphereflake_upright(
        bottom_left_corner + Point3::new(width / 1.5, radius, length / 2.0),
        radius, 
        mirror,
        5
    );

    world.add_hittable(sphereflake);

    cam.look_from = Point3::new(0.0, 0.0, length * 2.70);
    cam.look_at = Point3::zero();
    cam.vfov = 35.0;

    Arc::new(BVH::new(&mut world))
}

pub fn book2_final_scene(cam: &mut Camera) -> Arc<HittableSync> {
    let mut world = HittableComposite::new();
    let ground_material = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side: usize = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_in_range(1.0, 101.0);
            let z1 = z0 + w;

            world.add_hittable(box_new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground_material.clone()
            ))
        }
    }

    let light = DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0));
    world.add_hittable(Arc::new(
        Quad::new(
            Point3::new(123.0,554.0,147.0),
             Vec3::new(300.0, 0.0,0.0), 
             Vec3::new(0.0,0.0,265.0),
              Arc::new(light)
    )));

    // * All the various spheres
    // *=========================

    world.add_hittable(new_sphereflake_upright(
        Point3::new(400.0, 400.0, 200.0),
        50.0,  
        Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1))),
        3
    ));

    world.add_hittable(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric{ir: 1.5})
    )));

    world.add_hittable(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new_fuzzy(Color::new(0.8, 0.8, 0.9)))
    )));

    let mut boundry = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric{ir: 1.5})
    ));

    world.add_hittable(boundry.clone());
    world.add_hittable(Arc::new(ConstantMedium::new_color(
        boundry, 
        0.2, 
        Color::new(0.2, 0.4, 0.9)
    )));

    boundry = Arc::new(Sphere::new(Point3::
        zero(), 
        5000.0, 
        Arc::new(Dielectric{ir: 1.5})
    ));
    world.add_hittable(Arc::new(ConstantMedium::new_color(
        boundry, 
        0.0001, 
        COLOR_WHITE
    )));

    // earth
    let earth_texture = ImageTexture::new("./resources/textures/earthmap.jpg");
    let emat = Lambertian::new_texture(Arc::new(earth_texture));
    world.add_hittable(Arc::new(Sphere::new(
        Point3::new(400.0,200.0,400.0),
        100.0,
        Arc::new(emat)
    )));

    // noisy sphere
    let per_texture = Arc::new(NoiseTexture::new(0.2));
    world.add_hittable(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0), 
        80.0, 
        Arc::new(Lambertian::new_texture(per_texture))
    )));
        

    // Sphery box
    let mut lumpy_box = HittableComposite::new();
    for _ in 0..1000 {
        lumpy_box.add_hittable(Arc::new(
            Sphere::new(
                Point3::random_range(0.0, 165.0),
                10.0,
                Arc::new(Lambertian::new(COLOR_WHITE * 0.73))
            )
        ));
    }
    world.add_hittable(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BVH::new(&mut lumpy_box)),
            15.0)),
        Vec3::new(-100.0,270.0,395.0)
    )));

    cam.look_from = Point3::new(478.0, 278.0, -600.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.vfov = 40.0;
    cam.background = COLOR_BLACK;

    Arc::new(BVH::new(&mut world))
}

pub fn sphereflake_on_sandy_plane(cam: &mut Camera) -> Arc<HittableSync> {
    let mut world = HittableComposite::new();
    let plane_size = 30.0;

    // Big sand colored plane
    world.add_hittable(Arc::new(generate_default_plane(
        plane_size,
        Some(Color::new(0.99607843, 0.87843137, 0.60784314) * Color::new(2.0, 2.0, 1.0 ))
    )));

    world.add_hittable(new_sphereflake_upright(
        Point3::new(0.0, 1.0, 0.0),
        1.0, 
        Arc::new(Metal::new_mirror(Color::new(0.2, 0.2, 0.2))),
        5
    ));

    cam.look_from = Point3::new(-3.0, 5.0, -8.0);
    cam.look_at = Point3::new(0.0, 1.0, 0.0);
    cam.background = Color::new(0.5, 0.6, 1.0);

    Arc::new(BVH::new(&mut world))
}