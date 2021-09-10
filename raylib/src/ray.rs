use crate::hittable::Hittables;
use crate::random_in_unit_sphere;
use crate::sphere::Sphere;
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
        self.origin.add(&self.direction.mul(t))
    }

    pub fn diffused_world_color(&self, world: &Hittables<Sphere>, max_depth: i32) -> Vec3 {
        if max_depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        match world.hit(self, 0.0, f64::INFINITY) {
            Some(point) => {
                let target = point.at.add(&point.normal).add(&random_in_unit_sphere());
                let ray = Ray::new(point.at, target.sub(&point.at));

                ray.diffused_world_color(&world, max_depth - 1).mul(0.5)
            }
            None => self.color(),
        }
    }

    pub fn world_color(&self, world: &Hittables<Sphere>) -> Vec3 {
        match world.hit(self, 0.0, f64::INFINITY) {
            Some(point) => point.normal.add(&Vec3::new(1.0, 1.0, 1.0)).mul(0.5),
            None => self.color(),
        }
    }

    pub fn color(&self) -> Vec3 {
        let dir = self.direction.unit();
        let t = 0.5 * (dir.y + 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0);
        let blue = Vec3::new(0.5, 0.7, 1.0);

        // linear interpolation (lerp)
        // blendedValue = (1−t) ⋅ startValue + t ⋅ endValue
        white.mul(1.0 - t).add(&blue.mul(t))
    }

    // hit_sphere calculates whether or not a ray from the camera origin
    // will hit a sphere with the given center and radius
    //
    // P(t): the ray from the camera to the plane
    // C: the center of the sphere
    // r: the sphere's radius
    //
    // (P(t)−C)⋅(P(t)−C)=r^2
    // gives
    // (A+t*b−C)⋅(A+t*b−C)=r^2
    // where
    // P(t)=A+t*b
    // expanded
    // t^2*b⋅b+2t*b⋅(A−C)+(A−C)⋅(A−C)−r^2=0
    // where * is scalar multiplication and ⋅ is the dot product
    //
    // discriminant
    // for a'x^2 + b'x + c'
    // disc = b'^2 - 4a'c'
    // giving
    // a' = b⋅b
    // b' = 2*b⋅(A−C)
    // c' = (A−C)⋅(A−C)−r^2
    //
    // simplification
    // b' = 2*h
    // gives
    // (-h +/- (h^2 - ac) ^ (1/2)) / a
    // where h = b⋅(A−C)
    // giving discriminant
    // (h^2 - ac)
    pub fn hit_sphere(&self, center: Vec3, radius: f64) -> f64 {
        let oc = self.origin.sub(&center);

        // vector dotted with itself is equal to squared length of the vector
        //
        // let a = self.direction.dot(&self.direction); // a' above
        let a = self.direction.length_squared(); // a' above

        // using b' = 2*h simplification
        //
        // let b = 2.0 * self.direction.dot(&oc); // b' above
        let h = self.direction.dot(&oc); // h above

        // vector dotted with itself is equal to squared length of the vector
        //
        // let c = oc.dot(&oc) - radius * radius;
        let c = oc.length_squared() - radius * radius;

        // let discriminant = b * b - 4.0 * a * c;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return -1.0;
        }

        // using b' = 2*h simplification
        //
        // return (-b - discriminant.sqrt()) / (2.0 * a);
        return (-h - discriminant.sqrt()) / a;
    }
}
