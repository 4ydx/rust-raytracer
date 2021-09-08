extern crate raylib;

use raylib::{file::File, vec::Vec3};

fn main() {
    let width = 256;
    let height = 128;

    let output = File::new("example.ppm", height, width);
    for h in (0..height).rev() {
        println!("Scan height: {}", height - h);
        for w in 0..width {
            let r = (w as f64 / (width as f64 - 1.0)) * 255.999;
            let g = (h as f64 / (height as f64 - 1.0)) * 255.999;
            let b = (0.25) * 255.999;
            let v = Vec3::new(r, g, b);
            output.write(v);
        }
    }
    println!("DONE")
}
