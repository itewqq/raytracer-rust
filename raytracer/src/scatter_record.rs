use crate::{Ray, Vec3};

pub enum ScatterRecord{
    Specular {
        specular_ray: Ray,
        attenuation: Vec3,
    },
    Diffuse {
        scattered: Ray,
        attenuation: Vec3,
    }
}