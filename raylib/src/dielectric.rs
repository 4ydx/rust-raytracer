use crate::hittable::Hit;
use crate::material::Material;
use crate::random;
use crate::ray::Ray;
use crate::vec::Vec3;

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

#[derive(Clone)]
pub struct Dielectric10_2 {
    pub index_of_refraction: f64,
}

#[derive(Clone)]
pub struct Dielectric10_3 {
    pub index_of_refraction: f64,
}

#[derive(Clone)]
pub struct Dielectric10_4 {
    pub index_of_refraction: f64,
}

impl Dielectric10_2 {
    pub fn new(index_of_refraction: f64) -> Dielectric10_2 {
        Dielectric10_2 {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric10_2 {
    fn scatter(
        &self,
        ray: &Ray,
        hit: Hit,
        _rng: &mut rand::rngs::ThreadRng,
    ) -> Option<(Ray, Vec3)> {
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

impl Dielectric10_3 {
    pub fn new(index_of_refraction: f64) -> Dielectric10_3 {
        Dielectric10_3 {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric10_3 {
    fn scatter(
        &self,
        ray: &Ray,
        hit: Hit,
        _rng: &mut rand::rngs::ThreadRng,
    ) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64;
        if hit.front_face {
            refraction_ratio = 1.0 / self.index_of_refraction;
        } else {
            refraction_ratio = self.index_of_refraction;
        }
        let unit_direction = ray.direction.unit();

        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract {
            direction = unit_direction.reflect(hit.normal);
        } else {
            direction = unit_direction.refract(hit.normal, refraction_ratio);
        }

        let scattered = Ray::new(hit.point, direction);

        Some((scattered, attenuation))
    }
}

impl Dielectric10_4 {
    pub fn new(index_of_refraction: f64) -> Dielectric10_4 {
        Dielectric10_4 {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric10_4 {
    fn scatter(&self, ray: &Ray, hit: Hit, rng: &mut rand::rngs::ThreadRng) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64;
        if hit.front_face {
            refraction_ratio = 1.0 / self.index_of_refraction;
        } else {
            refraction_ratio = self.index_of_refraction;
        }
        let unit_direction = ray.direction.unit();

        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random(rng) {
            direction = unit_direction.reflect(hit.normal);
        } else {
            direction = unit_direction.refract(hit.normal, refraction_ratio);
        }

        let scattered = Ray::new(hit.point, direction);

        Some((scattered, attenuation))
    }
}
