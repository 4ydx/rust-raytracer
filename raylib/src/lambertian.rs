use crate::material::Material;
use crate::random_unit_vector;
use crate::vec::Vec3;

struct Lambertian {
    albedo: Vec3,
}

impl Material for Lambertian {
    pub fn scatter(&self, ray: Ray, hit: Hit) -> Option<(Ray, Vec3)> {
        let scatter_direction = hit.normal + random_unit_vector();
        let scattered = Ray::new(hit.point, scatter_direction);
        return Some(scattered, self.albedo);
    }
}
