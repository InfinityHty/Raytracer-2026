use crate::vec3::Vec3;
use std::rc::Rc;
pub trait Texture {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Vec3;
}
pub struct SolidColor {
    albedo: Vec3,
}
impl SolidColor {
    pub fn new(albedo: Vec3) -> SolidColor {
        SolidColor { albedo }
    }
}
impl Texture for SolidColor {
    #[allow(unused_variables)]
    fn value(&self, u: f64, v: f64, point: Vec3) -> Vec3 {
        self.albedo
    }
}
pub struct CheckeredTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
    scale: f64,
}
impl CheckeredTexture {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>, scale: f64) -> CheckeredTexture {
        CheckeredTexture { odd, even, scale }
    }
}
impl Texture for CheckeredTexture {
    #[allow(unused_variables)]
    fn value(&self, u: f64, v: f64, point: Vec3) -> Vec3 {
        let pos_x = (point.x * 1.0 / self.scale).floor() as i32;
        let pos_y = (point.y * 1.0 / self.scale).floor() as i32;
        let pos_z = (point.z * 1.0 / self.scale).floor() as i32;

        if (pos_x + pos_y + pos_z) % 2 == 0 {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}
