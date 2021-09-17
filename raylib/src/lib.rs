use rand::prelude::*;

pub mod camera;
pub mod dielectric;
pub mod file;
pub mod hittable;
pub mod lambertian;
pub mod material;
pub mod metal;
pub mod ray;
pub mod sphere;
pub mod vec;

pub fn random(rng: &mut rand::rngs::ThreadRng) -> f64 {
    rng.gen()
}

pub fn random_between(min: f64, max: f64, rng: &mut rand::rngs::ThreadRng) -> f64 {
    min + (max - min) * random(rng)
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees * std::f64::consts::PI) / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }

    x
}

pub fn random_in_unit_disk(rng: &mut rand::rngs::ThreadRng) -> vec::Vec3 {
    loop {
        let p = vec::Vec3::new(
            random_between(-1.0, 1.0, rng),
            random_between(-1.0, 1.0, rng),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_hemisphere(normal: &vec::Vec3, rng: &mut rand::rngs::ThreadRng) -> vec::Vec3 {
    let in_unit_sphere = random_in_unit_sphere(rng);
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_unit_vector(rng: &mut rand::rngs::ThreadRng) -> vec::Vec3 {
    random_in_unit_sphere(rng).unit()
}

pub fn random_in_unit_sphere(rng: &mut rand::rngs::ThreadRng) -> vec::Vec3 {
    loop {
        let p = vec::Vec3::random_between(-1.0, 1.0, rng);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn write_color_03_3(output: &file::File, color: vec::Vec3) {
    output.write(vec::Vec3::new(
        255.999 * color.x,
        255.999 * color.y,
        255.999 * color.z,
    ));
}

pub fn write_color(
    output: &file::File,
    color: vec::Vec3,
    samples_per_pixel: i32,
    gamma_correct: bool,
) {
    if samples_per_pixel == 0 {
        let adjusted_color = color * 255.999;
        output.write(adjusted_color);
        return;
    }
    let scale = 1.0 / samples_per_pixel as f64;
    let mut adjusted_color = color * scale;
    if gamma_correct {
        adjusted_color = adjusted_color.sqrt();
    }
    adjusted_color.x = 256.0 * clamp(adjusted_color.x, 0.0, 0.999);
    adjusted_color.y = 256.0 * clamp(adjusted_color.y, 0.0, 0.999);
    adjusted_color.z = 256.0 * clamp(adjusted_color.z, 0.0, 0.999);

    output.write(adjusted_color);
}
