use crate::perlin::PerlinNoise;
use crate::vec3::Vec3;
use image::{DynamicImage, GenericImageView};
use std::rc::Rc;
pub trait Texture {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Vec3;
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
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Vec3 {
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
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Vec3 {
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
pub struct ImageTexture {
    image: DynamicImage,
}
impl ImageTexture {
    pub fn new(image: image::DynamicImage) -> ImageTexture {
        ImageTexture { image }
    }
}
impl Texture for ImageTexture {
    #[allow(unused_variables)]
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Vec3 {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;
        let pixel = self.image.get_pixel(i, j);
        Vec3::new(
            pixel[0] as f64 / 255.0,
            pixel[1] as f64 / 255.0,
            pixel[2] as f64 / 255.0,
        )
    }
}
pub struct NoiseTexture {
    pub noise: PerlinNoise,
    #[allow(dead_code)]
    pub scale: f64,
}
impl Texture for NoiseTexture {
    #[allow(unused_variables)]
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Vec3 {
        // let scale_point = *point * self.scale;
        // Vec3::new(1.0, 1.0, 1.0) * self.noise.turb(point, 7)
        Vec3::new(0.5, 0.5, 0.5)
            * (1.0 + (self.scale * point.z + 10.0 * self.noise.turb(point, 7)).sin())
    }
}
