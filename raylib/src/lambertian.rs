use crate::hittable::Hit;
use crate::material::Material;
use crate::random_unit_vector;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: Hit) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = hit.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }
        let scattered = Ray::new(hit.point, scatter_direction);

        return Some((scattered, self.albedo));
    }
}
