use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::random_percentage;
use crate::vec::Color3;
use crate::vec::Vec3;
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

pub trait Material: Debug + Send + Sync {
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color3, fuzz: f64) -> Self {
        assert!(fuzz >= 0.0);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut reflected = Vec3::reflect(&ray_in.dir, &hit_record.normal).unit();

        if self.fuzz > 0.0 {
            reflected = reflected + self.fuzz * Vec3::random_unit();
        }

        let scattered = Ray::new(hit_record.p, reflected);
        Some(ScatterRecord::new(scattered, self.albedo))
    }
}

#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    /// Schlick's approximation for reflectance
    fn reflectance(&self, cosine: f64) -> f64 {
        let r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let refraction_index = if hit_record.is_front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.dir.unit();
        let cos_theta = unit_direction.negate().dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction;
        let cannot_refract = refraction_index * sin_theta > 1.0;
        if cannot_refract || self.reflectance(cos_theta) > random_percentage() {
            // Cannot refract
            direction = Vec3::reflect(&unit_direction, &hit_record.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &hit_record.normal, refraction_index);
        }

        let scattered = Ray::new(hit_record.p, direction);
        Some(ScatterRecord::new(scattered, Color3::new(1.0, 1.0, 1.0)))
    }
}
