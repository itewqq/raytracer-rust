use crate::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64
}
