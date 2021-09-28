#![allow(dead_code)]
// You SHOULD remove above line in your code.

use crate::{Vec3, Point3, Color};
use crate::{Hittable, HittableList};
use crate::{Sphere, utils};
use crate::{Material, lambertian::Lambertian, metal::Metal, dielectric::Dielectric};
use std::sync::Arc;
use rand::{rngs::SmallRng, Rng, SeedableRng};
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
    let mut rng = SmallRng::from_entropy();

    // Add ground
    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mut spheres: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: material_ground.clone(),
        }),
    ];

    // Add random small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Point3::new(a as f64 + 0.9 * rng.gen_range(0.0..1.0), 0.2, b as f64 + 0.9 * rng.gen_range(0.0..1.0));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = match choose_mat {
                    x if x < 0.8 => {
                        let albedo = utils::random_in_unit_sphere(&mut rng);
                        let albedo = Vec3::elemul(albedo, albedo); // No negative color
                        Arc::new(Lambertian::new(albedo))
                    }
                    x if x < 0.95 => {
                        let albedo = Color::new(rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0));
                        let fuzz = rng.gen_range(0.0..0.5);
                        Arc::new(Metal::new(albedo, fuzz))
                    }
                    _ => {
                        Arc::new(Dielectric::new(1.5))
                    }
                };
                spheres.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material.clone(),
                }));
            }
        }
    }

    // Add big balls
    let big_material1 = Arc::new(Dielectric::new(1.5));
    let big_material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let big_material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    spheres.append(&mut vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: big_material1.clone(),
        }),
        Box::new(Sphere {
            center: Vec3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: big_material2.clone(),
        }),
        Box::new(Sphere {
            center: Vec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: big_material3.clone(),
        }),
    ]);

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
