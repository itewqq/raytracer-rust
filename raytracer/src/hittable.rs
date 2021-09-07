#[allow(unused_imports)]
use crate::{HitRecord, Point3, Ray, Vec3};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut t_closest = t_max;
        let mut hit_record: Option<HitRecord> = None;
        for hittable in &self.hittables {
            match hittable.hit(&ray, t_min, t_closest) {
                Some(hr) => {
                    t_closest = hr.t;
                    hit_record = Some(hr);
                }
                None => {}
            }
        }
        hit_record
    }
}
