extern crate raylib;

use raylib::{file::File, ray::Ray, vec::Vec3, write_color};

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as i64;
    println!("image h {} w {}", height, width);

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0; // distance b/t the projection plane and projection point
    println!("viewport h {} w {}", viewport_height, viewport_width);

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // render
    let output = File::new("example.ppm", height, width);
    for h in (0..height).rev() {
        println!("Scan height: {}", height - h);
        for w in 0..width {
            let u: f64 = w as f64 / (width as f64 - 1.0);
            let v: f64 = h as f64 / (height as f64 - 1.0);

            let direction = lower_left_corner + horizontal * u + vertical * v + origin;

            let ray = Ray {
                origin: origin,
                direction: direction,
            };

            let color: Vec3;
            let t = ray.hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5);
            if t > 0.0 {
                let normal = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
                color = Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5;
            } else {
                color = ray.color();
            }
            write_color(&output, color, 0, false);
        }
    }
    println!("DONE")
}
