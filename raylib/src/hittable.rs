use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Hit<'world> {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Option<&'world dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

impl<'world> Hit<'world> {
    pub fn new(
        t: f64,
        point: Vec3,
        normal: Vec3,
        front_face: bool,
        material: Option<&'world dyn Material>,
    ) -> Hit {
        Hit {
            t: t,
            point: point,
            normal: normal,
            front_face: front_face,
            material: material,
        }
    }
}

pub struct Hittables {
    pub list: std::vec::Vec<Box<dyn Hittable>>,
}

impl Hittables {
    pub fn new() -> Hittables {
        Hittables {
            list: std::vec::Vec::new(),
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest = t_max;
        let mut current_point = None;

        for obj in self.list.iter() {
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
