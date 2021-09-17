extern crate raylib;

use raylib::{file::File, vec::Vec3, write_color_03_3};

fn main() {
    let width = 256;
    let height = 128;

    let output = File::new("example.ppm", height, width);
    for h in (0..height).rev() {
        println!("Scan height: {}", height - h);
        for w in 0..width {
            let pixel_color = Vec3::new(
                w as f64 / (width as f64 - 1.0),
                h as f64 / (height as f64 - 1.0),
                0.25,
            );
            write_color_03_3(&output, pixel_color);
        }
    }
    println!("DONE")
}
