use lazy_static::lazy_static;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use crate::interval::Interval;
use crate::utils::random_f64;
use crate::utils::random_percentage;

pub type Point3 = Vec3;
pub type Color3 = Vec3;

lazy_static! {
    static ref INTENSITY: Interval = Interval::new(0.0, 0.999);
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn random() -> Self {
        Self {
            x: random_percentage(),
            y: random_percentage(),
            z: random_percentage(),
        }
    }

    pub fn random_interval(min: f64, max: f64) -> Self {
        assert!(min < max);
        Self {
            x: random_f64(min, max),
            y: random_f64(min, max),
            z: random_f64(min, max),
        }
    }

    pub fn random_unit() -> Self {
        loop {
            let p = Vec3::random_interval(-1.0, 1.0);
            let lensq = p.squared_length();
            if 1e-169 < lensq && lensq <= 1.0 {
                return p.unit();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Self {
        let on_unit_sphere = Vec3::random_unit();
        if on_unit_sphere.dot(&normal) > 0.0 {
            // In the same hemisphere as the normal
            return on_unit_sphere;
        }
        Vec3::zero() - on_unit_sphere
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(random_f64(-1.0, 1.0), random_f64(-1.0, 1.0), 0.0);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }

    pub fn reflect(vec: &Vec3, normal: &Vec3) -> Self {
        (*vec) - 2.0 * vec.dot(normal) * (*normal)
    }

    pub fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = uv.negate().dot(&normal).min(1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *normal);
        let r_out_parallel = -(1.0 - r_out_perp.squared_length()).abs().sqrt() * *normal;
        r_out_perp + r_out_parallel
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn negate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Color3 {
    pub fn write(&self, output: &mut String) {
        fn linear_to_gamma(linear_component: f64) -> f64 {
            if linear_component > 0.0 {
                return linear_component.sqrt();
            }
            0.0
        }

        let r = linear_to_gamma(self.x);
        let g = linear_to_gamma(self.y);
        let b = linear_to_gamma(self.z);

        let rbyte = INTENSITY.clamp(r) * 256.0;
        let gbyte = INTENSITY.clamp(g) * 256.0;
        let bbyte = INTENSITY.clamp(b) * 256.0;
        output.push_str(&format!(
            "{} {} {}\n",
            rbyte as usize, gbyte as usize, bbyte as usize
        ));
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {} {}", self.x, self.y, self.z))
    }
}
