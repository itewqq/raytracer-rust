#![allow(clippy::float_cmp)]

mod material;
mod ray;
mod scene;
mod vec3;

use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
pub use ray::Ray;
use scene::example_scene;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;
pub use vec3::{Color, Point3, Vec3};

pub struct World {
    pub height: u32,
}

impl World {
    pub fn color(&self, _x: u32, _y: u32) -> Color {
        unimplemented!();
    }

    pub fn ray_color(&self, ray: &Ray) -> Color {
        let disc = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
        let result = match disc > 0.0 {
            true => {
                let nv = (ray.at(disc) - Vec3::new(0.0, 0.0, -1.0)).unit();
                Color::new(nv.x + 1.0, nv.y + 1.0, nv.z + 1.0) * 0.5
            },
            false => {
                let t = 0.5 * (ray.direction.unit().y + 1.0);
                Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
            }
        };
        result * 255.999
    }
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - *center;
    let a = ray.direction.squared_length();
    let half_b = oc * ray.direction;
    let c = oc.squared_length() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant > 0.0 {
        (-half_b - discriminant.sqrt()) / a
    } else {
        -1.0
    }
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
                    let origin = Point3::new(0.0, 0.0, 0.0);
                    let u = (x as f64) / (width as f64 - 1.0);
                    let v = (y as f64) / (height as f64 - 1.0);
                    let direction = lower_left_corner + horizontal * u + vertical * v - origin;
                    let ray = Ray::new(origin, direction);
                    // let color = world_ptr.color(x, y);
                    let color = world_ptr.ray_color(&ray);
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
