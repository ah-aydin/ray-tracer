mod aabb;
mod bvh;
mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec;

use std::sync::Arc;

use hittable::HittableList;
use sphere::Sphere;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::material::Dielectric;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::utils::random_f64;
use crate::utils::random_percentage;
use crate::vec::Color3;
use crate::vec::Point3;
use crate::vec::Vec3;

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 1280;
    let samples_per_pixel: usize = 256; // Number of samples which will be used for aliasing
    let max_depth = 50; // Maximum number of times a ray will bounce
    let vfov = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let camera = Arc::new(Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        defocus_angle,
        focus_dist,
        true,
    ));

    let mut world = HittableList::new();

    let m_ground = Arc::new(Lambertian::new(Color3::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        m_ground,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let m = random_percentage();
            let center = Point3::new(
                a + 0.9 * random_percentage(),
                0.2,
                b + 0.9 * random_percentage(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if m < 0.8 {
                    // Make it bounce at time t=1
                    let center_t1 = center + Point3::new(0.0, random_percentage() * 0.2, 0.0);
                    // diffuse
                    let albedo = Color3::random() * Color3::random();
                    let mat = Arc::new(Lambertian::new(albedo));
                    world.add(Sphere::new_moving(center, center_t1, 0.2, mat));
                } else if m < 0.95 {
                    // metal
                    let r = random_f64(0.5, 1.0);
                    let albedo = Color3::new(r, r, r);
                    let fuzz = random_f64(0.0, 0.5);
                    let mat = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(center, 0.2, mat));
                } else {
                    // glass
                    let mat = Arc::new(Dielectric::new(1.5));
                    world.add(Sphere::new(center, 0.2, mat));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian::new(Color3::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Metal::new(Color3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    let bvh_root = BVHNode::new(&mut world);

    camera.render(Arc::new(bvh_root));
}
