use crate::{HitRecord, Ray, ScatterRecord};
use rand::rngs::SmallRng;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: HitRecord, rng: &mut SmallRng) -> Option<ScatterRecord>;
}
