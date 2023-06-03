mod vec3;

use std::io::{Write, stderr};

fn main() {
    // Image

    let img_width: i32 = 256;
    let img_height: i32 = 256;

    // Render

    // Header
    println!("P3\n{img_width} {img_height}\n255\n");

    let img_width_norm: f64 = f64::from(img_width - 1);
    let img_height_norm: f64 = f64::from(img_height - 1);
    let mut tri_start: i32;
    let mut tri_end: i32;
    let mut stderr = stderr();
    
    // Pixels
    for i in 0..img_height {
        tri_start = 127 - (i/2);
        tri_end = 127 + (i/2);
        eprint!("\rScanlines remaining: {}", img_height-i);
        stderr.flush().unwrap();

        for j in 0..img_width {
            let mut r: f64 = 0.0;
            let mut g: f64 = 0.0;
            let mut b: f64 = 0.0;

            if j >= tri_start && j < tri_end {
                r = f64::from(j) / img_width_norm;
                g = f64::from(255 - i) / img_height_norm;
                b = 0.25;
            }

            let ir: u8 = (r * 255.0) as u8;
            let ig: u8 = (g * 255.0) as u8;
            let ib: u8 = (b * 255.0) as u8;

            println!("{ir} {ig} {ib}")
        }
    }
    eprintln!("\nDone.")
}
