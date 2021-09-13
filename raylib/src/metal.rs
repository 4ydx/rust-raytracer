use crate::hittable::Hit;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Metal {
    pub albedo: Vec3,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: Hit) -> Option<(Ray, Vec3)> {
        let reflected = ray.direction.unit().reflect(&hit.normal);
        let scattered = Ray::new(hit.point, reflected);

        return Some((scattered, self.albedo));
    }
}
