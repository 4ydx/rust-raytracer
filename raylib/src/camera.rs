use crate::degrees_to_radians;
use crate::random_in_unit_disk;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub w: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub lens_radius: f64,
}

fn build_camera(viewport_width: f64, viewport_height: f64) -> Camera {
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    Camera {
        origin: origin,
        horizontal: horizontal,
        vertical: vertical,
        lower_left_corner: origin
            - horizontal / 2.0
            - vertical / 2.0
            - Vec3::new(0.0, 0.0, focal_length),
        w: Vec3::default(),
        u: Vec3::default(),
        v: Vec3::default(),
        lens_radius: 0.0,
    }
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        build_camera(viewport_width, viewport_height)
    }

    pub fn with_vfov(vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        build_camera(viewport_width, viewport_height)
    }

    pub fn from(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = view_up.cross(&w).unit();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - (horizontal / 2.0) - (vertical / 2.0) - w,
            w: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            lens_radius: 0.0,
        }
    }

    pub fn new_12_2(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = view_up.cross(&w).unit();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            w: w,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin,
        }
    }

    pub fn ray_12_2(&self, s: f64, t: f64, rng: &mut rand::rngs::ThreadRng) -> Ray {
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
        }
    }
}
