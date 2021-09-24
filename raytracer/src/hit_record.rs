use crate::{Material, Point3, Vec3};
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Arc<dyn Material>,
}
