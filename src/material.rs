use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::{Color3, Vec3};
use std::fmt::Debug;

#[derive(Debug)]
pub struct ScatterRecord {
    pub scattered: Ray,
    pub attenuation: Color3,
}

impl ScatterRecord {
    fn new(scattered: Ray, attenuation: Color3) -> Self {
        Self {
            scattered,
            attenuation,
        }
    }
}

pub trait Material: Debug {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<ScatterRecord> {
        None
    }
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Color3,
}

impl Lambertian {
    pub fn new(albedo: Color3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.p, scatter_direction);
        Some(ScatterRecord::new(scattered, self.albedo))
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Color3,
}

impl Metal {
    pub fn new(albedo: Color3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(&ray_in.dir, &hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected);
        Some(ScatterRecord::new(scattered, self.albedo))
    }
}
