use crate::vec::Point3;
use crate::vec::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Self {
            origin,
            dir,
            tm: 0.0,
        }
    }

    pub fn new_time(origin: Point3, dir: Vec3, tm: f64) -> Self {
        Self { origin, dir, tm }
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + t * self.dir;
    }
}
