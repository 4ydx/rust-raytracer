use crate::hittable::{Hit, Hittable};
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin.sub(&self.center);
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-h - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-h + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = point.sub(&self.center).div(self.radius);
        let front_face = ray.direction.dot(&outward_normal) < 0.0;

        let normal: Vec3;
        if front_face {
            normal = outward_normal;
        } else {
            normal = outward_normal.mul(-1.0);
        }
        return Some(Hit::new(root, point, normal, front_face));
    }
}
