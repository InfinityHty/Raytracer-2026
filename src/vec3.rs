// vec3 后续补充接口
use image::Rgb;
use rand::{RngExt, rng};
use std::ops;
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
    pub fn generate_rand_norm(min: f64, max: f64) -> Self {
        let mut rng = rng();
        loop {
            let x = rng.random_range(min..max);
            let y = rng.random_range(min..max);
            let z = rng.random_range(min..max);
            let vec = Vec3::new(x, y, z);
            let sq = vec.length_squared();
            if sq <= 1.0 && sq > 1e-160 {
                return vec / sq.sqrt();
            }
        }
    }
    pub fn generate_rand_unit_disk(min: f64, max: f64) -> Vec3 {
        let mut rng = rng();
        loop {
            let x = rng.random_range(min..max);
            let y = rng.random_range(min..max);
            let vec = Vec3::new(x, y, 0.0);
            let sq = vec.length_squared();
            if sq <= 1.0 {
                return vec;
            }
        }
    }
    pub fn near_zero(&self) -> bool {
        const EPSILON: f64 = 1e-8;
        if self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON {
            return true;
        }
        false
    }
    pub fn cross_multiply(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }
    pub fn to_rgb(self) -> Rgb<u8> {
        Rgb([self.x as u8, self.y as u8, self.z as u8])
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn normalize(&self) -> Vec3 {
        Vec3 {
            x: self.x / self.length_squared().sqrt(),
            y: self.y / self.length_squared().sqrt(),
            z: self.z / self.length_squared().sqrt(),
        }
    }
}
impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}
impl ops::Mul for Vec3 {
    type Output = f64;
    fn mul(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}
