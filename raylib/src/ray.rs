use crate::hittable::Hittables;
use crate::vec::Vec3;
use crate::{random_in_hemisphere, random_in_unit_sphere, random_unit_vector};

#[derive(Debug)]
pub struct Ray {
    pub direction: Vec3,
    pub origin: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn color_04_2(&self) -> Vec3 {
        let dir = self.direction.unit();
        let t = 0.5 * (dir.y + 1.0);

        // linear interpolation (lerp): blendedValue = (1−t) ⋅ startValue + t ⋅ endValue
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }

    pub fn color_05_2(&self) -> Vec3 {
        if self.hit_sphere_05_2(Vec3::new(0.0, 0.0, -1.0), 0.5) {
            return Vec3::new(1.0, 0.0, 0.0);
        }
        self.color_04_2()
    }

    pub fn color_06_1(&self) -> Vec3 {
        let t = self.hit_sphere_06_1(Vec3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let normal = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
            return Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5;
        }
        self.color_04_2()
    }

    pub fn color_06_2(&self) -> Vec3 {
        let t = self.hit_sphere_06_2(Vec3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let normal = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
            return Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5;
        }
        self.color_04_2()
    }

    pub fn color_06_7(&self, world: &Hittables) -> Vec3 {
        match world.hit(self, 0.0, f64::INFINITY) {
            Some(hit) => (hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5,
            None => self.color_04_2(),
        }
    }

    pub fn color_08_2(
        &self,
        world: &Hittables,
        depth: i32,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        match world.hit(self, 0.0, f64::INFINITY) {
            Some(hit) => {
                let target = hit.point + hit.normal + random_in_unit_sphere(rng);
                let ray = Ray::new(hit.point, target - hit.point);

                ray.color_08_2(&world, depth - 1, rng) * 0.5
            }
            None => self.color_04_2(),
        }
    }

    pub fn color_08_4(
        &self,
        world: &Hittables,
        depth: i32,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        match world.hit(self, 0.001, f64::INFINITY) {
            Some(hit) => {
                let target = hit.point + hit.normal + random_in_unit_sphere(rng);
                let ray = Ray::new(hit.point, target - hit.point);

                ray.color_08_4(&world, depth - 1, rng) * 0.5
            }
            None => self.color_04_2(),
        }
    }

    pub fn color_08_5(
        &self,
        world: &Hittables,
        depth: i32,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        match world.hit(self, 0.001, f64::INFINITY) {
            Some(hit) => {
                let target = hit.point + hit.normal + random_unit_vector(rng);
                let ray = Ray::new(hit.point, target - hit.point);

                ray.color_08_5(&world, depth - 1, rng) * 0.5
            }
            None => self.color_04_2(),
        }
    }

    pub fn color_08_6(
        &self,
        world: &Hittables,
        depth: i32,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        match world.hit(self, 0.001, f64::INFINITY) {
            Some(hit) => {
                let target = hit.point + random_in_hemisphere(&hit.normal, rng);
                let ray = Ray::new(hit.point, target - hit.point);

                ray.color_08_6(&world, depth - 1, rng) * 0.5
            }
            None => self.color_04_2(),
        }
    }

    pub fn color_09_4(
        &self,
        world: &Hittables,
        depth: i32,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        match world.hit(self, 0.001, f64::INFINITY) {
            Some(hit) => match hit.material {
                Some(material) => match material.scatter(self, hit, rng) {
                    Some((scattered, attenuation)) => {
                        return attenuation * scattered.color_09_4(&world, depth - 1, rng)
                    }
                    None => Vec3::new(0.0, 0.0, 0.0),
                },
                None => panic!("expecting material"),
            },
            None => self.color_04_2(),
        }
    }

    pub fn hit_sphere_05_2(&self, center: Vec3, radius: f64) -> bool {
        let oc = self.origin - center;
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * self.direction.dot(&oc);
        let c = oc.dot(&oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        discriminant > 0.0
    }

    pub fn hit_sphere_06_1(&self, center: Vec3, radius: f64) -> f64 {
        let oc = self.origin - center;
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * self.direction.dot(&oc);
        let c = oc.dot(&oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return -1.0;
        }
        (-b - discriminant.sqrt()) / (2.0 * a)
    }

    pub fn hit_sphere_06_2(&self, center: Vec3, radius: f64) -> f64 {
        let oc = self.origin - center;

        // vector dotted with itself is equal to squared length of the vector
        let a = self.direction.length_squared();

        // using b' = 2*h simplification
        let h = self.direction.dot(&oc);

        // vector dotted with itself is equal to squared length of the vector
        let c = oc.length_squared() - radius * radius;

        // substituting b = 2 * h
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return -1.0;
        }

        // substituting b = 2 * h
        return (-h - discriminant.sqrt()) / a;
    }
}
