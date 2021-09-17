extern crate raylib;

use rand::prelude::*;
use raylib::{
    camera::Camera, dielectric::Dielectric10_4, file::File, hittable::Hittables,
    lambertian::Lambertian, metal::Metal, random, random_between, sphere::Sphere, vec::Vec3,
    write_color,
};

fn random_scene(rng: &mut rand::rngs::ThreadRng) -> Hittables {
    let mut world: Hittables = Hittables::new();

    let ground = Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };
    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Box::new(ground)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random(rng);
            let center = Vec3::new(
                a as f64 + 0.9 * random(rng),
                0.2,
                b as f64 + 0.9 * random(rng),
            );

            let around = Vec3::new(4.0, 0.2, 0.0);
            if (center - around).length() > 0.9 {
                if choose_material < 0.8 {
                    let material = Lambertian::new(Vec3::random(rng) * Vec3::random(rng));
                    world
                        .list
                        .push(Box::new(Sphere::new(center, 0.2, Some(Box::new(material)))));
                } else if choose_material < 0.95 {
                    let material = Metal::new(
                        Vec3::random_between(0.5, 1.0, rng),
                        random_between(0.0, 0.5, rng),
                    );
                    world
                        .list
                        .push(Box::new(Sphere::new(center, 0.2, Some(Box::new(material)))));
                } else {
                    let material = Dielectric10_4::new(1.5);
                    world
                        .list
                        .push(Box::new(Sphere::new(center, 0.2, Some(Box::new(material)))));
                }
            }
        }
    }

    let m1 = Dielectric10_4::new(1.5);
    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Some(Box::new(m1)),
    )));
    let m2 = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
    world.list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Some(Box::new(m2)),
    )));
    let m3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Some(Box::new(m3)),
    )));

    world
}

fn main() {
    let mut rng = thread_rng();

    // image
    let aspect_ratio = 3.0 / 2.0;
    let width = 1200;
    let height = (width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 500;
    let max_depth = 50;
    println!("image h {} w {}", height, width);

    // camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new_12_2(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let world = random_scene(&mut rng);

    // render
    let output = File::new("example.ppm", height, width);
    for h in (0..height).rev() {
        println!("Scan height: {}", height - h);
        for w in 0..width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u: f64 = (w as f64 + random(&mut rng)) / (width as f64 - 1.0);
                let v: f64 = (h as f64 + random(&mut rng)) / (height as f64 - 1.0);
                let ray = camera.ray_12_2(u, v, &mut rng);
                pixel_color = pixel_color + ray.color_09_4(&world, max_depth, &mut rng);
            }
            write_color(&output, pixel_color, samples_per_pixel, true);
        }
    }
    println!("DONE")
}
