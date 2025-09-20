mod hittable;
mod interval;
mod ray;
mod sphere;
mod vec;

use std::fs::File;
use std::io::Write;

use hittable::HittableList;
use interval::Interval;
use ray::Ray;
use sphere::Sphere;
use vec::Color3;
use vec::Point3;
use vec::Vec3;

fn ray_color(ray: &Ray, hittable_list: &HittableList) -> Color3 {
    if let Some(hit_record) = hittable_list.hit(ray, Interval::new(0.0, f64::MAX)) {
        let normal = hit_record.normal;
        return Color3::new(normal.x, normal.y, normal.z);
    }

    let unit_direction = ray.dir.unit();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color3::new(1.0, 1.0, 1.0) + a * Color3::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 800;
    let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);
    let aspect_ratio = image_width as f64 / image_height as f64;

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = Point3::zero();

    // Calculate the vectors accross the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Create hittable objects
    let mut hittable_list = HittableList::new();
    hittable_list.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));

    // Render
    println!("Writing image to file");
    let mut image_data = String::new();
    image_data.push_str(&format!("P3\n{image_width} {image_height}\n255\n"));

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &hittable_list);
            pixel_color.write(&mut image_data);
        }
    }

    let mut file = File::create("image.ppm").expect("Failed to open image file");
    file.write(image_data.as_bytes())
        .expect("Failed while writing to file");
    println!("Done");
}
