use crate::{Color, Ray, HitRecord, Material, ScatterRecord};
use crate::utils::random_unit_vector;
use rand::rngs::SmallRng;

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: HitRecord, rng: &mut SmallRng)  -> Option<ScatterRecord> {
        let scatter_dir = hit_record.normal + random_unit_vector(rng);
        let scattered = Ray{origin: hit_record.p, direction: scatter_dir};
        let attenuation = self.albedo;
        return Some(ScatterRecord::Diffuse{
            scattered,
            attenuation,
        });
    }
}