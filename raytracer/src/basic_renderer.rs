#[allow(dead_code)]
#[allow(unused_imports)]

use crate::clamp3;
use crate::Ray;
use crate::{Color, Vec3};
use crate::{Hittable, HittableList};
use image::{ImageBuffer, Rgb, RgbImage};

pub struct BasicRenderer {
    pub anti_aliasing: u32,
}

impl BasicRenderer {
    fn ray_color(world: &HittableList, ray: &Ray) -> Color {
        let rec_option = world.hit(ray, 0.0, f64::INFINITY);
        let result = match rec_option {
            Some(rec) => {
                let nv = (ray.at(rec.t) - Vec3::new(0.0, 0.0, -1.0)).unit();
                Color::new(nv.x + 1.0, nv.y + 1.0, nv.z + 1.0) * 0.5
            }
            None => {
                let t = 0.5 * (ray.direction.unit().y + 1.0);
                Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
            }
        };
        clamp3(result) * 255.999
    }

    pub fn render() -> RgbImage {
        unimplemented!()
    }
}
