use std::rc::Rc;

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Point3;
use crate::vec::Vec3;

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub is_front_face: bool,
}

impl HitRecord {
    /// `outward_normal` must be a unit vector
    pub fn new(
        p: Point3,
        outward_normal: Vec3,
        ray: &Ray,
        material: Rc<dyn Material>,
        t: f64,
    ) -> Self {
        Self {
            p,
            normal: outward_normal,
            material,
            t,
            is_front_face: ray.dir.dot(&outward_normal) < 0.0,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut current_hit_record: Option<HitRecord> = None;
        for object in &self.objects {
            let current_max = ray_t
                .max
                .min(current_hit_record.as_ref().map(|r| r.t).unwrap_or(f64::MAX));
            if let Some(hit_record) = object.hit(ray, Interval::new(ray_t.min, current_max)) {
                current_hit_record = Some(hit_record)
            }
        }
        current_hit_record
    }
}
