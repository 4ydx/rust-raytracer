extern crate raylib;

use raylib::{
    camera::Camera, file::File, hittable::Hittables, sphere::Sphere, vec::Vec3, write_color,
};

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as i64;
    println!("image h {} w {}", height, width);

    // camera from 7.2
    let camera = Camera::new();

    // world
    let mut world: Hittables = Hittables {
        list: std::vec::Vec::new(),
    };
    world
        .list
        .push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, None)));
    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        None,
    )));

    // render
    let output = File::new("example.ppm", height, width);
    for h in (0..height).rev() {
        println!("Scan height: {}", height - h);
        for w in 0..width {
            let u: f64 = w as f64 / (width as f64 - 1.0);
            let v: f64 = h as f64 / (height as f64 - 1.0);
            let ray = camera.ray(u, v);
            write_color(&output, ray.world_color(&world), 0, false);
        }
    }
    println!("DONE")
}
