use rand::prelude::*;

pub mod camera;
pub mod file;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vec;

pub fn random() -> f64 {
    let mut rng = thread_rng();
    rng.gen()
}

pub fn random_between(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}

pub fn random_in_unit_sphere() -> vec::Vec3 {
    loop {
        let p = vec::Vec3::random(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn write_color(output: &file::File, color: vec::Vec3, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let adjusted_color = color.mul(scale);
    output.write(adjusted_color);
}
