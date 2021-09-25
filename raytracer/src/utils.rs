use crate::{Color, Point3, Vec3};
use rand::{rngs::SmallRng, Rng};

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    } else {
        return x;
    }
}

pub fn clamp3(color: Color) -> Color {
    Color {
        x: clamp(color.x, 0.0, 0.999),
        y: clamp(color.y, 0.0, 0.999),
        z: clamp(color.z, 0.0, 0.999),
    }
}

pub fn random_unit_vector(rng: &mut SmallRng) -> Point3 {
    random_in_unit_sphere(rng).unit() // TODO
}

pub fn random_in_hemisphere(normal: Vec3, rng: &mut SmallRng) -> Point3 {
    let in_unit_sphere = random_in_unit_sphere(rng);
    if in_unit_sphere * normal > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn random_in_unit_sphere(rng: &mut SmallRng) -> Point3 {
    loop {
        let tmp = Point3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        if tmp.length() < 1.0 {
            return tmp;
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (v * n) * 2.0
}

pub fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv * normal).min(1.0_f64);
    let r_out_perp = (uv + normal * cos_theta) * etai_over_etat;
    let r_out_parallel = -normal * (1.0 - r_out_perp.squared_length()).abs().sqrt();
    r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}
