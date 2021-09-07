use crate::{HitRecord, Hittable, Ray, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let half_b = oc * ray.direction;
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            // Find the nearest root that lies in the acceptable range.
            let t = (-half_b - discriminant.sqrt()) / a; // smaller t
            if t_min < t && t < t_max {
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { p, normal, t });
            }
            let t = (-half_b + discriminant.sqrt()) / a; // larger t
            if t_min < t && t < t_max {
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { p, normal, t });
            }
        }
        return None;
    }
}
