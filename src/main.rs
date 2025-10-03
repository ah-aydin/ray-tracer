mod camera;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec;

use hittable::HittableList;
use sphere::Sphere;

use crate::camera::Camera;
use crate::vec::Vec3;

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 800;
    let samples_per_pixel: usize = 100;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel);

    // Create hittable objects
    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    camera.render(world);
}
