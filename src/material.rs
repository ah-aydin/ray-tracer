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
    /// ## Math
    /// ### Concept
    /// Diffuse (Lambertian) surfaces scatter light uniformly in all directions.
    /// Instead of reflecting rays in a mirror-like way, they bounce light randomly
    /// from the hit point within a unit hemisphere oriented by the surface normal.
    ///
    /// ### Variables
    /// - `N` → surface normal at the hit point
    /// - `P` → hit point
    /// - `d` → random unit vector (representing diffuse scatter direction)
    ///
    /// ### Calculation
    /// The scattered ray direction is chosen as:
    /// `scatter_direction = N + random_unit_vector()`
    ///
    /// This randomizes direction within the hemisphere of the normal,
    /// producing the classic "matte" appearance.
    ///
    /// If the result is near zero (numerically unstable), the normal itself is used
    /// as the direction to avoid degenerate vectors.
    ///
    /// ### Outcome
    /// - `attenuation` = surface color (albedo)
    /// - `scattered` = ray starting at `P` with direction `scatter_direction`
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new_time(hit_record.p, scatter_direction, ray_in.tm);
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
    /// ## Math
    /// ### Concept
    /// Metallic surfaces reflect rays according to the law of reflection,
    /// with an optional "fuzz" factor that blurs the reflection.
    ///
    /// ### Variables
    /// - `v` → incoming ray direction
    /// - `N` → surface normal
    /// - `r` → reflected direction
    /// - `fuzz` → controls the deviation from perfect reflection (0 = mirror)
    ///
    /// ### Calculation
    /// Perfect reflection direction:
    /// `r = v - 2 * (v ⋅ N) * N`
    ///
    /// If `fuzz > 0`, add a small random perturbation within a unit sphere:
    /// `r = r + fuzz * random_unit_vector()`
    ///
    /// The result is a slightly blurred reflection depending on the fuzz value.
    ///
    /// ### Outcome
    /// - `attenuation` = surface color (albedo)
    /// - `scattered` = ray starting at hit point, moving in `r`
    /// - The material does not absorb light; it reflects it directionally.
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut reflected = Vec3::reflect(&ray_in.dir, &hit_record.normal).unit();

        if self.fuzz > 0.0 {
            reflected = reflected + self.fuzz * Vec3::random_unit();
        }

        let scattered = Ray::new_time(hit_record.p, reflected, ray_in.tm);
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
    /// ## Math
    /// ### Concept
    /// Dielectric materials (like glass or water) can both reflect and refract light.
    /// Whether a ray reflects or refracts depends on the incident angle and the
    /// material's index of refraction.
    ///
    /// ### Variables
    /// - `n₁` → refractive index of the medium the ray comes from
    /// - `n₂` → refractive index of the material
    /// - `η` = `n₁ / n₂` → ratio of refractive indices
    /// - `θᵢ` → incident angle
    /// - `θₜ` → transmission (refraction) angle
    /// - `cosθ` = `-v ⋅ N`
    ///
    /// ### Calculation
    /// Using Snell's Law: `η * sinθᵢ = sinθₜ`
    /// If `η * sinθᵢ > 1`, total internal reflection occurs (no refraction possible).
    ///
    /// Otherwise, we probabilistically choose reflection or refraction based on
    /// **Schlick's approximation**:
    /// `R(θ) ≈ R₀ + (1 - R₀) * (1 - cosθ)⁵`
    /// where `R₀ = ((1 - n₂) / (1 + n₂))²`
    ///
    /// ### Outcome
    /// - If total internal reflection or reflection probability `R(θ)` is high → reflect
    /// - Otherwise → refract using Snell’s law
    ///
    /// In either case:
    /// - `attenuation` = white (no absorption)
    /// - `scattered` = new ray with reflected or refracted direction
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

        let scattered = Ray::new_time(hit_record.p, direction, ray_in.tm);
        Some(ScatterRecord::new(scattered, Color3::new(1.0, 1.0, 1.0)))
    }
}
