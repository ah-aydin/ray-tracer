use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::hittable::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::random_u64;

type BoxCompareFn = fn(&dyn Hittable, &dyn Hittable) -> bool;

fn box_compare(left: &dyn Hittable, right: &dyn Hittable, axis: usize) -> bool {
    let left_axis_interval = left.boundnig_box().axis_interval(axis);
    let right_axis_interval = right.boundnig_box().axis_interval(axis);
    return left_axis_interval.min < right_axis_interval.min;
}

fn box_compare_x(left: &dyn Hittable, right: &dyn Hittable) -> bool {
    box_compare(left, right, 0)
}

fn box_compare_y(left: &dyn Hittable, right: &dyn Hittable) -> bool {
    box_compare(left, right, 1)
}

fn box_compare_z(left: &dyn Hittable, right: &dyn Hittable) -> bool {
    box_compare(left, right, 2)
}

/// Bounding Volume Hierarchy Node
pub struct BVHNode {
    bbox: AABB,
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}

impl BVHNode {
    pub fn new(hittable_list: &mut HittableList) -> BVHNode {
        let end = hittable_list.get_objects().len();
        BVHNode::new_span(hittable_list.get_objects(), 0, end)
    }

    fn new_span(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> BVHNode {
        let comparator: BoxCompareFn = match random_u64(0, 2) {
            0 => box_compare_x,
            1 => box_compare_y,
            2 => box_compare_z,
            _ => unreachable!(),
        };
        let object_span = end - start;

        let left;
        let right;
        if object_span == 1 {
            left = Arc::clone(&objects[start]);
            right = Arc::clone(&objects[start]);
        } else if object_span == 2 {
            left = Arc::clone(&objects[start]);
            right = Arc::clone(&objects[start + 1]);
        } else {
            objects[start..end].sort_by(|left, right| {
                if comparator(left.as_ref(), right.as_ref()) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });

            let mid = (start + end) / 2;
            left = Arc::new(BVHNode::new_span(objects, start, mid));
            right = Arc::new(BVHNode::new_span(objects, mid, end));
        }

        BVHNode {
            bbox: AABB::from_boxes(left.boundnig_box(), right.boundnig_box()),
            left,
            right,
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, &ray_t) {
            return None;
        }

        let left_hit_record = self.left.hit(ray, ray_t.clone());
        let interval = if let Some(HitRecord { t, .. }) = left_hit_record {
            Interval::new(ray_t.min, t)
        } else {
            Interval::new(ray_t.min, ray_t.max)
        };
        let right_hit_record = self.right.hit(ray, interval);

        right_hit_record.or_else(|| left_hit_record)
    }

    fn boundnig_box(&self) -> &AABB {
        &self.bbox
    }
}
