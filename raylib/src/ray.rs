use crate::hittable::Hittables;
use crate::random_in_hemisphere;
use crate::vec::Vec3;

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

    pub fn color(&self) -> Vec3 {
        let dir = self.direction.unit();
        let t = 0.5 * (dir.y + 1.0);

        // linear interpolation (lerp)
        // blendedValue = (1−t) ⋅ startValue + t ⋅ endValue
        let white = Vec3::new(1.0, 1.0, 1.0);
        let blue = Vec3::new(0.5, 0.7, 1.0);
        white * (1.0 - t) + blue * t
    }

    pub fn diffused_world_color_in_hemisphere(
        &self,
        world: &Hittables,
        max_depth: i32,
        world_hit_t_min: f64,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Vec3 {
        if max_depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        match world.hit(self, world_hit_t_min, f64::INFINITY) {
            Some(hit) => {
                let target = hit.point + random_in_hemisphere(&hit.normal, rng);
                let ray = Ray::new(hit.point, target - hit.point);

                ray.diffused_world_color_in_hemisphere(&world, max_depth - 1, world_hit_t_min, rng)
                    * 0.5
            }
            None => self.color(),
        }
    }

    pub fn diffused_world_color(
        &self,
        world: &Hittables,
        max_depth: i32,
        world_hit_t_min: f64,
        random_vec3: fn(&mut rand::rngs::ThreadRng) -> Vec3,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Vec3 {
        if max_depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        match world.hit(self, world_hit_t_min, f64::INFINITY) {
            Some(hit) => match hit.material {
                Some(material) => match material.scatter(self, hit, rng) {
                    Some((scattered, attenuation)) => {
                        return attenuation
                            * scattered.diffused_world_color(
                                &world,
                                max_depth - 1,
                                world_hit_t_min,
                                random_vec3,
                                rng,
                            )
                    }
                    None => Vec3::new(0.0, 0.0, 0.0),
                },
                None => {
                    let target = hit.point + hit.normal + random_vec3(rng);
                    let ray = Ray::new(hit.point, target - hit.point);

                    ray.diffused_world_color(
                        &world,
                        max_depth - 1,
                        world_hit_t_min,
                        random_vec3,
                        rng,
                    ) * 0.5
                }
            },
            None => self.color(),
        }
    }

    pub fn world_color(&self, world: &Hittables) -> Vec3 {
        match world.hit(self, 0.0, f64::INFINITY) {
            Some(point) => (point.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5,
            None => self.color(),
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

    pub fn hit_sphere(&self, center: Vec3, radius: f64) -> f64 {
        let oc = self.origin - center;

        // vector dotted with itself is equal to squared length of the vector
        // let a = self.direction.dot(&self.direction); // a' above
        let a = self.direction.length_squared(); // a' above

        // using b' = 2*h simplification
        // let b = 2.0 * self.direction.dot(&oc); // b' above
        let h = self.direction.dot(&oc); // h above

        // vector dotted with itself is equal to squared length of the vector
        // let c = oc.dot(&oc) - radius * radius;
        let c = oc.length_squared() - radius * radius;

        // let discriminant = b * b - 4.0 * a * c;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return -1.0;
        }

        // using b' = 2*h simplification
        // return (-b - discriminant.sqrt()) / (2.0 * a);
        return (-h - discriminant.sqrt()) / a;
    }
}
