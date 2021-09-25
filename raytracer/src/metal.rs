use crate::{Color, Ray, HitRecord, Material, ScatterRecord, utils::{random_in_unit_sphere, reflect}};
use rand::rngs::SmallRng;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {albedo, fuzz}
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: HitRecord, rng: &mut SmallRng)  -> Option<ScatterRecord> {
        let reflected = reflect(ray_in.direction.unit(), hit_record.normal);
        let specular_ray = Ray::new(hit_record.p, reflected + random_in_unit_sphere(rng) * self.fuzz);
        let attenuation = self.albedo;
        if specular_ray.direction * hit_record.normal > 0.0 {
            return Some(ScatterRecord::Specular{
                specular_ray,
                attenuation,
            });
        }else{
            return None;
        }
    }
}