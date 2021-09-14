use crate::{Color, Point3};
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