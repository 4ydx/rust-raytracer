use crate::hittable::Hit;
use crate::ray::Ray;
use crate::vec::Vec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: Hit, rng: &mut rand::rngs::ThreadRng) -> Option<(Ray, Vec3)>;
}
