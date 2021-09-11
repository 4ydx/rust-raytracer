use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Default, Debug)]
pub struct Hit {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

impl Hit {
    pub fn new(t: f64, point: Vec3, normal: Vec3, front_face: bool) -> Hit {
        Hit {
            t: t,
            point: point,
            normal: normal,
            front_face: front_face,
        }
    }
}

pub struct Hittables<T: Hittable>(pub std::vec::Vec<T>);

impl<T> Hittables<T>
where
    T: Hittable,
{
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest = t_max;
        let mut current_point = None;

        for obj in self.0.iter() {
            match obj.hit(&ray, t_min, closest) {
                Some(point) => {
                    closest = point.t;
                    current_point = Some(point);
                }
                None => {}
            }
        }
        current_point
    }
}
