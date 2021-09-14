use crate::hittable::Hit;
use crate::material::Material;
use crate::random;
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Clone)]
pub struct Dielectric {
    pub index_of_refraction: f64,
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
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

        let cos_theta = unit_direction.mul(-1.0).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random() {
            direction = unit_direction.reflect(&hit.normal);
        } else {
            direction = unit_direction.refract(&hit.normal, refraction_ratio);
        }

        let scattered = Ray::new(hit.point, direction);

        Some((scattered, attenuation))
    }
}
