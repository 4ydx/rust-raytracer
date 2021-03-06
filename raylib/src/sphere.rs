use crate::hittable::{Hit, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Option<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Option<Box<dyn Material>>) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
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

        let outward_normal = (point - self.center) / self.radius;
        let mut normal = outward_normal;

        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        if !front_face {
            normal = -outward_normal;
        }

        // https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref
        return Some(Hit::new(
            root,
            point,
            normal,
            front_face,
            self.material.as_deref(),
        ));
    }
}
