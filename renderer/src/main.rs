fn main() {
    // Image

    let img_width: i32 = 256;
    let img_height: i32 = 256;

    // Render

    // Header
    println!("P3\n{img_width} {img_height}\n255\n");

    let img_width_norm: f64 = f64::from(img_width - 1);
    let img_height_norm: f64 = f64::from(img_height - 1);

    // Pixels
    for i in (0..img_height).rev() {
        for j in 0..img_width {
            let r: f64 = f64::from(j) / img_width_norm;
            let g: f64 = f64::from(i) / img_height_norm;
            let b: f64 = 0.25;

            let ir: u8 = (r * 255.0) as u8;
            let ig: u8 = (g * 255.0) as u8;
            let ib: u8 = (b * 255.0) as u8;

            println!("{ir} {ig} {ib}")
        }
    }
}
