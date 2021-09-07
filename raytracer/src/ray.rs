use crate::{Point3, Vec3};

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(3.0, 4.0, 5.0)),
            Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(3.0, 4.0, 5.0))
        );
    }

    #[test]
    fn test_at() {
        let r1 = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(r1.at(3.0), Vec3::new(10.0, 14.0, 18.0));
    }
}
