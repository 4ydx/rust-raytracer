use crate::hittable::Hit;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: Hit) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64;
        if hit.front_face {
            refraction_ratio = 1.0 / self.index_of_refraction;
        } else {
            refraction_ratio = self.index_of_refraction;
        }
        let unit_direction = ray.direction.unit();
        let refracted = unit_direction.refract(hit.normal, refraction_ratio);
        let scattered = Ray::new(hit.point, refracted);

        Some((scattered, attenuation))
    }
}
