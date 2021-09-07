#![allow(clippy::float_cmp)]

mod material;
mod ray;
mod scene;
mod vec3;
mod hit_record;
mod hittable;
mod sphere;

use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use scene::example_scene;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;
pub use vec3::{Color, Point3, Vec3};
pub use ray::Ray;
pub use hit_record::HitRecord;
pub use hittable::{Hittable, HittableList};
pub use sphere::Sphere;


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
    result * 255.999
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

    let height = 512;
    let width = 1024;
    let aspect_ratio = (width as f64) / (height as f64);

    // create a channel to send objects between threads
    let (tx, rx) = channel();
    let pool = ThreadPool::new(n_workers);

    let bar = ProgressBar::new(n_jobs as u64);

    // use Arc to pass one instance of World to multiple threads
    let world = Arc::new(example_scene());

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width as f64, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height as f64, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ptr = world.clone();
        pool.execute(move || {
            // here, we render some of the rows of image in one thread
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
                    let u = (x as f64) / (width as f64 - 1.0);
                    let v = (y as f64) / (height as f64 - 1.0);
                    let direction = lower_left_corner + horizontal * u + vertical * v - origin;
                    let ray = Ray::new(origin, direction);
                    let color = ray_color(&world_ptr, &ray);
                    *pixel = Rgb([color.x as u8, color.y as u8, color.z as u8]);
                }
            }
            // send row range and rendered image to main thread
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
        });
    }

    let mut result: RgbImage = ImageBuffer::new(width, height);

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
