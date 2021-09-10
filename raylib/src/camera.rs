use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin
                .sub(&horizontal.div(2.0))
                .sub(&vertical.div(2.0))
                .sub(&Vec3::new(0.0, 0.0, focal_length)),
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self
                .lower_left_corner
                .add(&self.horizontal.mul(u))
                .add(&self.vertical.mul(v))
                .sub(&self.origin),
        }
    }
}