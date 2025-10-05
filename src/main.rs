mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec;

use std::f64::consts::PI;
use std::rc::Rc;

use hittable::HittableList;
use sphere::Sphere;

use crate::camera::Camera;
use crate::material::Dielectric;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::vec::Color3;
use crate::vec::Point3;
use crate::vec::Vec3;

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 800;
    let samples_per_pixel: usize = 100; // Number of samples which will be used for aliasing
    let max_depth = 10; // Maximum number of times a ray will bounce
    let vfov = 110.0;
    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
    );

    // let m_ground = Rc::new(Lambertian::new(Color3::new(0.8, 0.8, 0.0)));
    // let m_center = Rc::new(Lambertian::new(Color3::new(0.1, 0.2, 0.5)));
    // let m_left = Rc::new(Dielectric::new(1.5));
    // let m_bubble = Rc::new(Dielectric::new(1.0 / 1.5));
    // let m_right = Rc::new(Metal::new(Color3::new(0.8, 0.6, 0.2), 0.15));
    //
    // let mut world = HittableList::new();
    // world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, m_ground));
    // world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, m_center));
    // world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, m_left));
    // world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, m_bubble));
    // world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, m_right));

    let m_left = Rc::new(Lambertian::new(Color3::new(0.0, 0.0, 1.0)));
    let m_right = Rc::new(Lambertian::new(Color3::new(1.0, 0.0, 0.0)));

    let radius = (PI / 4.0).cos();

    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(-radius, 0.0, -1.0), radius, m_left));
    world.add(Sphere::new(Point3::new(radius, 0.0, -1.0), radius, m_right));

    camera.render(world);
}
