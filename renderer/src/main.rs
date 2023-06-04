mod vec3;

use std::io::{Write, stderr};

fn main() {
    // Image

    let img_width: i32 = 256;
    let img_height: i32 = 256;
    let img_max_color: f64 = 255.0;

    // Render

    // Header
    println!("P3\n{img_width} {img_height}\n{img_max_color}\n");

    let img_width_norm: f64 = f64::from(img_width - 1);
    let img_height_norm: f64 = f64::from(img_height - 1);
    let mut tri_start: i32;
    let mut tri_end: i32;
    let mut stderr = stderr();
    
    // Pixels
    for i in 0..img_height {
        tri_start = (img_width / 2) - (i/2);
        tri_end = (img_width / 2) + (i/2);
        eprint!("\rScanlines remaining: {}", img_height-i);
        stderr.flush().unwrap();

        for j in 0..img_width {
            let mut r: f64 = 0.0;
            let mut g: f64 = 0.0;
            let mut b: f64 = 0.0;

            if j >= tri_start && j < tri_end {
                r = f64::from(j) / img_width_norm;
                g = f64::from(img_height - i) / img_height_norm;
                b = 0.25;
            }

            let ir: u8 = (r * img_max_color) as u8;
            let ig: u8 = (g * img_max_color) as u8;
            let ib: u8 = (b * img_max_color) as u8;

            println!("{ir} {ig} {ib}")
        }
    }
    eprintln!("\nDone.");

    let v1: vec3::Vec3 = vec3::Vec3::origin();
    let v2: vec3::Vec3 = vec3::Vec3::new(1.0, 2.0, 3.0);

    eprintln!("v1: {} {} {}", v1[0], v1[1], v1[2]);
    eprintln!("v2: {} {} {}", v2.x(), v2.y(), v2.z());
}
