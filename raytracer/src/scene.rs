#![allow(dead_code)]
// You SHOULD remove above line in your code.

use crate::{Vec3, Color};
use crate::{Hittable, HittableList};
use crate::Sphere;
use crate::{lambertian::Lambertian, metal::Metal, dielectric::Dielectric};
use std::sync::Arc;
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
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Dielectric::new(1.5));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    // Just for test
    let mut spheres: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: material_center.clone(),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: material_ground.clone(),
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_left.clone(),
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_right.clone(),
        }),
    ];

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
