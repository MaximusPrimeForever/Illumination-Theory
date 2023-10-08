mod math;
mod graphics;
mod geometry;
mod rendering;
mod scenes;

use std::env;
use std::sync::Arc;

use graphics::Camera;
use rendering::render::render_scene;
// use buffer::write_img_ppm;

use math::vec3::{Vec3, Point3, Color};


fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        scenes::test_scene();
        return Ok(());
    }
    if args.len() != 7 {
        panic!("Invalid arguments");
    }

    let image_width = args[1].parse::<usize>().unwrap();
    let aspect_ratio = args[2].parse::<f64>().unwrap();
    let vfov = args[3].parse::<f64>().unwrap();
    let samples_per_pixel: usize = args[4].parse::<usize>().unwrap();
    let trace_depth: usize = args[5].parse::<usize>().unwrap();
    let core_count: usize = args[6].parse::<usize>().unwrap();

    // debug - because I didn't bother to check how to pass arguments to the
    // vscode rust debugger
    // let image_width = 400;
    // let aspect_ratio = 16.0 / 9.0;
    // let vfov: f64 = 20.0;
    // let samples_per_pixel: usize = 50;
    // let trace_depth: usize = 10;
    // let core_count: usize = 1;

    // Control some parameters of camera from CLI
    let mut cam = Camera::default();
    cam.image_width = image_width;
    cam.image_height = (image_width as f64 / aspect_ratio) as usize;
    cam.vfov = vfov;
    
    // World
    // World functions configure camera position, orientation, focus, etc.
    // let world = rtweekend::one_weekend_endgame(8);
    // let world = rtweekend::cool_effects(8, 1.3);
    // let world = rtweekend::lit_world_textures(&mut cam);
    // let world = rtweekend::two_checkered_spheres(&mut cam);
    // let world = rtweekend::earth(&mut cam);
    // let world = rtweekend::tiled_texture(&mut cam);
    // let world = rtweekend::quad_scene(&mut cam);
    // let world = rtweekend::sphereflake_on_sandy_plane(&mut cam);
    // let world = rtweekend::quad_shadow_test(&mut cam);
    // let world = rtweekend::cornell_box(&mut cam);
    let world = scenes::cornell_box_dark_sphereflake(&mut cam);
    // let world = rtweekend::lit_world(&mut cam);

    // Must be called!
    cam.initialize();
    
    // Render
    let image_canvas = render_scene(
        core_count,
        world,
        Arc::new(cam),
        samples_per_pixel,
        trace_depth
    );
    
    // Output to file
    image_canvas.save_png("output.png");

    Ok(())
}
