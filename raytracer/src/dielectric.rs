use crate::{utils::random_in_unit_sphere, Color, HitRecord, Material, Ray, ScatterRecord, Vec3};
use rand::rngs::SmallRng;

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        let refraction_ratio = match hit_record.front {
            true => 1.0 / self.ir,
            false => self.ir,
        };
        let unit_direction = ray_in.direction.unit();
        let refracted = Vec3::refract(unit_direction, hit_record.normal, refraction_ratio);
        let specular_ray = Ray::new(hit_record.p, refracted);
        let attenuation = Color::new(1.0, 1.0, 1.0);
        return Some(ScatterRecord::Specular {
            specular_ray,
            attenuation,
        });
    }
}
