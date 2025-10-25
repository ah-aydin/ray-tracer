use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec::Point3;

/// Axis-Aligned Bounding Box
#[derive(Debug)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    /// `p1` and `p2` are the opposite corners of the bounding box
    pub fn from_points(p1: Point3, p2: Point3) -> Self {
        Self {
            x: if p1.x < p2.x {
                Interval::new(p1.x, p2.x)
            } else {
                Interval::new(p2.x, p1.x)
            },
            y: if p1.y < p2.y {
                Interval::new(p1.y, p2.y)
            } else {
                Interval::new(p2.y, p1.y)
            },
            z: if p1.z < p1.z {
                Interval::new(p1.z, p2.z)
            } else {
                Interval::new(p2.z, p1.z)
            },
        }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!("You're living in a higher dimention"),
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        let mut ray_t = ray_t.clone();
        let ray_origin = ray.origin;
        let ray_dir = ray.dir;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_origin[axis]) * adinv;
            let t1 = (ax.max - ray_origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }

    pub fn from_boxes(box1: &AABB, box2: &AABB) -> AABB {
        AABB {
            x: Interval::from_intervals(&box1.x, &box2.x),
            y: Interval::from_intervals(&box1.y, &box2.y),
            z: Interval::from_intervals(&box1.z, &box2.z),
        }
    }
}
