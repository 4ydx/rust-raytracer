use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Default, Debug)]
pub struct Hit {
    pub t: f64,
    pub at: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> (bool, Hit);
}

impl Hit {
    pub fn new(t: f64, at: Vec3, normal: Vec3, front_face: bool) -> Hit {
        Hit {
            t: t,
            at: at,
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
        let mut hit_anything = false;
        let mut closest = t_max;
        let mut current_at = Hit::default();

        for obj in self.0.iter() {
            let (is_hit, at) = obj.hit(&ray, t_min, closest);
            if is_hit {
                hit_anything = true;
                closest = at.t;
                current_at = at;
            }
        }
        if hit_anything {
            return Some(current_at);
        }
        None
    }
}
