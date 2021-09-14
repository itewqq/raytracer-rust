#![allow(dead_code)]
// You SHOULD remove above line in your code.

use crate::Vec3;
use crate::{Hittable, HittableList};
use crate::Sphere;
// use raytracer_codegen::make_spheres_impl;

// Call the procedural macro, which will become `make_spheres` function.
// make_spheres_impl! {}

// These three structs are just written here to make it compile.
// You should `use` your own structs in this file.
// e.g. replace next two lines with
// `use crate::materials::{DiffuseLight, ConstantTexture}`
pub struct ConstantTexture(Vec3);
pub struct DiffuseLight(ConstantTexture);

pub fn example_scene() -> HittableList {
    // Just for test
    let mut spheres: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0
        }),
    ]; // Now `spheres` stores two spheres.

    // let mut hittables = vec![]; // This is wrong
    let mut hittables: Vec<Box<dyn Hittable>> = vec![];
    // You can now add spheres to your own world
    hittables.append(&mut spheres);

    return HittableList{
        hittables,
    };
    // hittable_list.clear();
    // World { height: 512 }
}
