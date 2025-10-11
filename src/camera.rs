use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::thread;

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
    defocus_angle: f64,   // Varaition angle of rays through each pixel
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
    enable_motion_blur: bool,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        samples_per_pixel: usize,
        max_depth: usize,
        vfov: f64,
        look_from: Point3, // Point camera is looking from
        look_at: Point3,   // Point camera is looking at
        v_up: Vec3,        // Camera relative "up" direction
        defocus_angle: f64,
        focus_dist: f64, // Distance from camera lookfrom point to plane of perfect focus
        enable_motion_blur: bool,
    ) -> Camera {
        let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);
        let aspect_ratio = image_width as f64 / image_height as f64;

        let center = look_from;

        // Camera
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).unit(); // Unit vector pointing to the opposite of view direction (since right-hand coordinates are used)
        let u = v_up.cross(w).unit(); // Unit vector poniting to the right of the camera
        let v = w.cross(u); // Unit vector pointint to camera up

        // Calculate the vectors accross the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * v.negate();

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = center - viewport_u / 2.0 - viewport_v / 2.0 - (focus_dist * w);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

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
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            enable_motion_blur,
        }
    }

    pub fn render(self: Arc<Self>, objects: HittableList) {
        println!("Writing image to file");
        let mut image_data = String::new();
        image_data.push_str(&format!(
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        ));

        let thread_count = num_cpus::get().saturating_sub(4).max(1); // Using only 20 cores out of 24 that I have
        let batch_size = self.image_height / thread_count;
        let last_batch_size = self.image_height - batch_size * (thread_count - 1);

        let objects = Arc::new(objects);

        let mut thread_handles = Vec::new();
        for t in 0..thread_count {
            let batch_start = t * batch_size;
            let batch_end = if t == thread_count - 1 {
                batch_start + last_batch_size
            } else {
                batch_start + batch_size
            };

            let s = Arc::clone(&self);
            let objects = Arc::clone(&objects);
            let handle = thread::spawn(move || {
                let mut image_data = String::new();
                for j in batch_start..batch_end {
                    for i in 0..s.image_width {
                        let mut pixel_color = Color3::zero();
                        for _ in 0..s.samples_per_pixel {
                            let ray = s.get_ray(i, j);
                            pixel_color = pixel_color + s.ray_color(ray, &objects, s.max_depth);
                        }
                        pixel_color = pixel_color * s.pixel_sample_scale;
                        pixel_color.write(&mut image_data);
                    }
                }
                image_data
            });

            thread_handles.push(handle);
        }

        for th in thread_handles {
            let thread_data = th.join().unwrap();
            image_data.push_str(&thread_data);
        }

        let mut file = File::create("image.ppm").expect("Failed to open image file");
        file.write(image_data.as_bytes())
            .expect("Failed while writing to file");
        println!("Done");
    }

    /// Construct a camera ray originating from the defocus disk and directed at a randomly
    /// sampled point around the pixel location i, j.
    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = Vec3::new(random_percentage() - 0.5, random_percentage() - 0.5, 0.0);
        let pixel_center = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            // Get defocus disk sample
            let p = Vec3::random_in_unit_disk();
            self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
        };

        let ray_direction = pixel_center - ray_origin;
        if self.enable_motion_blur {
            Ray::new_time(ray_origin, ray_direction, random_percentage())
        } else {
            Ray::new(ray_origin, ray_direction)
        }
    }

    fn ray_color(&self, ray: Ray, objects: &HittableList, depth: usize) -> Color3 {
        // Bounce limit exceeded
        if depth <= 0 {
            return Color3::zero();
        }

        if let Some(hit_record) = objects.hit(&ray, Interval::new(0.001, f64::MAX)) {
            if let Some(scatter_record) = hit_record.material.scatter(&ray, &hit_record) {
                return scatter_record.attenuation
                    * self.ray_color(scatter_record.scattered, objects, depth - 1);
            }
            return Color3::zero();
        }

        // Color of the sky
        let unit_direction = ray.dir.unit();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color3::new(1.0, 1.0, 1.0) + a * Color3::new(0.5, 0.7, 1.0)
    }
}
