use crate::hittable::Hit;
use crate::material::Material;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        let mut f = fuzz;
        if f > 1.0 {
            f = 1.0;
        }
        Metal {
            albedo: albedo,
            fuzz: f,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: Hit) -> Option<(Ray, Vec3)> {
        let reflected = ray.direction.unit().reflect(&hit.normal);
        let scattered = Ray::new(
            hit.point,
            reflected.add(&random_in_unit_sphere().mul(self.fuzz)),
        );
        return Some((scattered, self.albedo));
    }
}
