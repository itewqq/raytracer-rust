use crate::{utils, Color, HitRecord, Material, Ray, ScatterRecord};
use rand::{rngs::SmallRng, Rng};

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
        let cos_theta = (-unit_direction * hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0
            || utils::reflectance(cos_theta, refraction_ratio) > _rng.gen_range(0.0..1.0);
        let direction = match cannot_refract {
            true => utils::reflect(unit_direction, hit_record.normal),
            false => utils::refract(unit_direction, hit_record.normal, refraction_ratio),
        };
        let specular_ray = Ray::new(hit_record.p, direction);
        let attenuation = Color::new(1.0, 1.0, 1.0);
        return Some(ScatterRecord::Specular {
            specular_ray,
            attenuation,
        });
    }
}
