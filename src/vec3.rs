// vec3 后续补充接口
use std::ops;
use image::Rgb;
#[derive(Debug,Clone,Copy)]
pub struct Vec3{
    x: f64,
    y: f64,
    z: f64,
}
impl Vec3{
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Self{x,y,z}
    }
    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb([self.x as u8,self.y as u8,self.z as u8])
    }
    pub fn get_x(&self) -> f64{
        self.x
    }
    pub fn get_y(&self) -> f64{
        self.y
    }
    pub fn get_z(&self) -> f64{
        self.z
    }
    pub fn length_squared(&self) -> f64{
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn normalize(&self) -> Vec3{
        Vec3{
            x: self.x / self.length_squared().sqrt(),
            y: self.y / self.length_squared().sqrt(),
            z: self.z / self.length_squared().sqrt(),
        }
    }
}
impl std::ops::Add for Vec3{
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3{
        Vec3{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl ops::Sub for Vec3{
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3{
        Vec3{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3{
        Vec3{
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}
impl ops::Mul for Vec3{
    type Output = f64;
    fn mul(self, other: Vec3) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
impl ops::Div<f64> for Vec3{
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3{
        Vec3{
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}