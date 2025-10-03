use std::fs::File;
use std::io::Write;

use crate::hittable::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::random_percentage;
use crate::vec::Color3;
use crate::vec::Point3;
use crate::vec::Vec3;

pub struct Camera {
    image_width: usize,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: usize,
    pixel_sample_scale: f64,
    max_depth: usize,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Camera {
        let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);
        let aspect_ratio = image_width as f64 / image_height as f64;

        // Camera
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let center = Point3::zero();

        // Calculate the vectors accross the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_sample_scale: 1.0 / (samples_per_pixel as f64),
            max_depth,
        }
    }

    pub fn render(&self, objects: HittableList) {
        println!("Writing image to file");
        let mut image_data = String::new();
        image_data.push_str(&format!(
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        ));

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color3::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(ray, &objects, self.max_depth);
                }
                pixel_color = pixel_color * self.pixel_sample_scale;
                pixel_color.write(&mut image_data);
            }
        }

        let mut file = File::create("image.ppm").expect("Failed to open image file");
        file.write(image_data.as_bytes())
            .expect("Failed while writing to file");
        println!("Done");
    }

    /// Construct a camera ray originating from the origin and directed at randomly sampled
    /// point around the pixel location i, j.
    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = Vec3::new(random_percentage() - 0.5, random_percentage() - 0.5, 0.0);
        let pixel_center = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_direction = pixel_center - self.center;
        Ray::new(self.center, ray_direction)
    }

    fn ray_color(&self, ray: Ray, objects: &HittableList, depth: usize) -> Color3 {
        if depth <= 0 {
            return Color3::zero();
        }

        if let Some(hit_record) = objects.hit(&ray, Interval::new(0.001, f64::MAX)) {
            let bounce_direction = Vec3::random_on_hemisphere(hit_record.normal);
            return 0.5
                * self.ray_color(Ray::new(hit_record.p, bounce_direction), objects, depth - 1);
        }

        let unit_direction = ray.dir.unit();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color3::new(1.0, 1.0, 1.0) + a * Color3::new(0.5, 0.7, 1.0)
    }
}
