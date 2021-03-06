#![allow(clippy::float_cmp)]

mod camera;
mod hit_record;
mod hittable;
mod lambertian;
mod dielectric;
mod metal;
mod material;
mod ray;
mod scatter_record;
mod scene;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
pub use hit_record::HitRecord;
pub use hittable::{Hittable, HittableList};
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use material::Material;
use rand::{rngs::SmallRng, Rng, SeedableRng};
pub use ray::Ray;
use scatter_record::ScatterRecord;
use scene::example_scene;
pub use sphere::Sphere;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;
use utils::{clamp3, random_in_hemisphere};
pub use vec3::{Color, Point3, Vec3};

fn gamma2_correct(color: Color, samples_per_pixel: u32) -> Color {
    let scale = 1.0 / (samples_per_pixel as f64);
    Color {
        x: (color.x * scale).sqrt(),
        y: (color.y * scale).sqrt(),
        z: (color.z * scale).sqrt(),
    }
}

fn ray_color(world: &HittableList, ray: &Ray, depth: u32, rng: &mut SmallRng) -> Color {
    if depth == 0 {
        return Color::zero();
    }
    let rec_option = world.hit(ray, 1e-5, f64::INFINITY);
    let result = match rec_option {
        Some(rec) => {
            let material = rec.material.clone();
            match material.scatter(ray, rec, rng) {
                Some(ScatterRecord::Specular {
                    specular_ray,
                    attenuation,
                }) => {
                    Vec3::elemul(ray_color(world, &specular_ray, depth - 1, rng), attenuation)
                }
                Some(ScatterRecord::Diffuse {
                    scattered,
                    attenuation,
                }) => {
                    Vec3::elemul(ray_color(world, &scattered, depth - 1, rng), attenuation)
                }
                None => {
                    Color::new(0.0, 0.0, 0.0)
                }
            }
            // let target = rec.p + random_in_hemisphere(rec.normal, rng);
            // ray_color(world, &Ray::new(rec.p, target - rec.p), depth - 1, rng) * 0.5
        }
        None => {
            let t = 0.5 * (ray.direction.unit().y + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    };
    return clamp3(result);
}

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn main() {
    // get environment variable CI, which is true for GitHub Action
    let is_ci = is_ci();

    // jobs: split image into how many parts
    // workers: maximum allowed concurrent running threads
    let (n_jobs, n_workers): (usize, usize) = if is_ci { (32, 2) } else { (16, 2) };

    println!(
        "CI: {}, using {} jobs and {} workers",
        is_ci, n_jobs, n_workers
    );

    // create a channel to send objects between threads
    let (tx, rx) = channel();
    let pool = ThreadPool::new(n_workers);

    // Progress bar
    let bar = ProgressBar::new(n_jobs as u64);

    // Image
    let height = 800;
    let width = 1200;
    let aspect_ratio = (width as f64) / (height as f64);

    // use Arc to pass one instance of World to multiple threads
    let world = Arc::new(example_scene());

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Arc::new(Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus));

    // Render
    let samples_per_pixel = 500;
    let max_depth = 50;

    println!("Start");

    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ptr = world.clone();
        let camera_ptr = camera.clone();
        pool.execute(move || {
            // here, we render some of the rows of image in one thread
            let mut rng = SmallRng::from_entropy();
            let row_begin = height as usize * i / n_jobs;
            let row_end = height as usize * (i + 1) / n_jobs;
            let render_height = row_end - row_begin;
            // img is a partial image
            let mut img: RgbImage = ImageBuffer::new(width, render_height as u32);
            for x in 0..width {
                // img_y is the row in partial rendered image
                // y is real position in final image
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = y as u32;
                    let pixel = img.get_pixel_mut(x, img_y as u32);
                    let mut color = Color::zero();
                    for _ in 0..samples_per_pixel {
                        let target_x: f64 = x as f64 + rng.gen_range(0.0..1.0);
                        let target_y: f64 = y as f64 + rng.gen_range(0.0..1.0);
                        let u = target_x / (width as f64 - 1.0);
                        let v = target_y / (height as f64 - 1.0);
                        let ray = camera_ptr.get_ray(u, v, &mut rng);
                        color += ray_color(&world_ptr, &ray, max_depth, &mut rng);
                    }
                    color = gamma2_correct(color, samples_per_pixel) * 255.999;
                    *pixel = Rgb([color.x as u8, color.y as u8, color.z as u8]);
                }
            }
            // send row range and rendered image to main thread
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
        });
    }

    let mut result: RgbImage = ImageBuffer::new(width, height);

    println!("Wait for result..");

    for (rows, data) in rx.iter().take(n_jobs) {
        // idx is the corrsponding row in partial-rendered image
        for (idx, row) in rows.enumerate() {
            for col in 0..width {
                let row = row as u32;
                let idx = idx as u32;
                // *result.get_pixel_mut(col, row) = *data.get_pixel(col, idx);
                *result.get_pixel_mut(col, height - 1 - row) = *data.get_pixel(col, idx);
                // Be consistent with the book
            }
        }
        bar.inc(1);
    }

    result.save("output/test.png").unwrap();
    bar.finish();
}
