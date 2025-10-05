use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Point3;

#[derive(Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        assert!(radius >= 0.0);
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    /// ## Math
    /// ### Variables
    /// `C(Cx, Cy, Cz)` is the sphere center
    /// `r` is the sphere radius
    /// `P(x, y, z)` point
    /// Ray: `P(t)= O + t*d` where `O` is origin and `d` is direction (point `P` is located on this ray)
    /// ### Calculation
    /// `(Cx - x)^2 + (Cy - y)^2 + (Cz - z)^2 = r^2` means point is on the surface of the sphere
    /// The above equation can be written as
    /// `(C - P) ⋅(C - P) = r^2`
    ///
    /// Replace `P` with `P(t)` since the point is a point on the ray
    /// `(C - P(t)) ⋅ (C - P(t)) = r^2`
    ///
    /// After replacing `P(t)` with `O + t*d` and extracting the common factors and moving `r^2` to the other side we endup with
    /// `t^2 * d ⋅d - t * 2 * d ⋅ (C - O) + (C - O) ⋅(C - O) - r^2 = 0`
    ///
    /// We endup with a quadratic formula where:
    /// `a = d ⋅d`
    /// `b = 2 * d ⋅ oc`
    /// `c = oc ⋅oc - r^2`
    /// with `t` as our variable and `oc = C - O`.
    ///
    /// ### Outcomes
    /// - If there are 0 roots, then the ray does not intersect the sphere
    /// - If there is 1 root, then the ray is a tangent to the surface of the sphere
    /// - If there are 2 roots, then the ray passes through the sphere
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.dir.squared_length(); // Squared length of a vector is the dot procut between a vector and itself
        // let b = -(2.0 * ray.dir.dot(&oc));
        let h = ray.dir.dot(&oc); // b = -2h to simply the formula for `discriminant`
        let c = oc.squared_length() - self.radius.powi(2);
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let root = (h - discriminant.sqrt()) / a; // Get the minimum root
        if !ray_t.surrounds(root) {
            return None;
        }

        let hit_point = ray.at(root);
        // This normal will always point outward
        let normal = (hit_point - self.center) / self.radius; // division by radius will make it a unit vector
        Some(HitRecord::new(
            hit_point,
            normal,
            ray,
            Arc::clone(&self.material),
            root,
        ))
    }
}
