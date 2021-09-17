extern crate raylib;

use rand::prelude::*;
use raylib::{
    camera::Camera, file::File, hittable::Hittables, lambertian::Lambertian, random,
    sphere::Sphere, vec::Vec3, write_color,
};

fn main() {
    let mut rng = thread_rng();

    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 100;
    let max_depth = 50;
    println!("image h {} w {}", height, width);

    // camera
    let camera = Camera::new11_1(90.0, aspect_ratio);

    // world
    let left = Lambertian::new(Vec3::new(0.0, 0.0, 1.0));
    let right = Lambertian::new(Vec3::new(1.0, 0.0, 0.0));

    let mut world: Hittables = Hittables {
        list: std::vec::Vec::new(),
    };
    let r = (std::f64::consts::PI / 4.0).cos();

    world.list.push(Box::new(Sphere::new(
        Vec3::new(-r, 0.0, -1.0),
        r,
        Some(Box::new(left)),
    )));

    world.list.push(Box::new(Sphere::new(
        Vec3::new(r, 0.0, -1.0),
        r,
        Some(Box::new(right)),
    )));

    // render
    let output = File::new("example.ppm", height, width);
    for h in (0..height).rev() {
        println!("Scan height: {}", height - h);
        for w in 0..width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u: f64 = (w as f64 + random(&mut rng)) / (width as f64 - 1.0);
                let v: f64 = (h as f64 + random(&mut rng)) / (height as f64 - 1.0);
                let ray = camera.ray(u, v);
                pixel_color = pixel_color + ray.color_09_4(&world, max_depth, &mut rng);
            }
            write_color(&output, pixel_color, samples_per_pixel, true);
        }
    }
    println!("DONE")
}
