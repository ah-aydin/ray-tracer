use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::vec::Color3;

pub struct Camera {}

impl Camera {
    pub fn new() -> Camera {
        Camera {}
    }

    pub fn render(&self, objects: HittableList) {
        todo!()
    }

    fn init(&mut self) {
        todo!()
    }

    fn ray_color(&self, ray: &Ray, objects: &HittableList) -> Color3 {
        todo!()
    }
}
