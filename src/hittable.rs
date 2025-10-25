use std::sync::Arc;

use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Point3;
use crate::vec::Vec3;

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub is_front_face: bool,
}

impl HitRecord {
    /// `outward_normal` must be a unit vector
    pub fn new(
        p: Point3,
        outward_normal: Vec3,
        ray: &Ray,
        material: Arc<dyn Material>,
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

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn boundnig_box(&self) -> &AABB;
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec![],
            bbox: AABB::from_points(Point3::zero(), Point3::zero()),
        }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.bbox = AABB::from_boxes(&self.bbox, object.boundnig_box());
        self.objects.push(Arc::new(object));
    }

    pub fn get_objects(&mut self) -> &mut Vec<Arc<dyn Hittable>> {
        &mut self.objects
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
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

    fn boundnig_box(&self) -> &AABB {
        todo!()
    }
}
