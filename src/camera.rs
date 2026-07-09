use crate::vec3::*;
use crate::hittable_list::*;
use crate::hittable::*;
use crate::ray::*;
use crate::interval::*;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use console::style;
pub struct Camera {
    aspect_ration: f64,
    width: u32,
}
impl Camera {
    pub fn new(aspect_ration: f64, width: u32 ) -> Self {
        Self { aspect_ration, width }
    }
    pub fn render(&self,world: &HittableList) {
        // 保存路径
        let path = std::path::Path::new("output/book1/image5.png");
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
        // 相机内参
        let eye_point: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let f = 1.0;

        let height = (self.width as f64 / self.aspect_ration) as u32;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.width as f64 / height as f64);

        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let viewport_u = Vec3::new(viewport_width as f64, 0.0, 0.0);

        let pixel_u = viewport_u / self.width as f64;
        let pixel_v = viewport_v / height as f64;

        let upper_left = eye_point - Vec3::new(0.0, 0.0, f) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00 = upper_left + pixel_u * 0.5 + pixel_v * 0.5;

        let mut img: RgbImage = ImageBuffer::new(self.width as u32, height);

        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((height * self.width as u32) as u64)
        };
        // 逐像素渲染
        for j in 0..height {
            for i in 0..self.width as u32 {
                let pixel_ij = pixel_00 + pixel_u * i as f64 + pixel_v * j as f64;
                // println!("{}",pixel_ij.get_y());
                let ray_direction = pixel_ij - eye_point;
                let ray = Ray::new(&eye_point, &ray_direction);
                let pixel = img.get_pixel_mut(i, j);

                *pixel = Camera::ray_color(&ray,&world);
            }
            progress.inc(1);
        }
        progress.finish();

        println!(
            "Output image as \"{}\"",
            style(path.to_str().unwrap()).yellow()
        );
        img.save(path).expect("Cannot save the image to the file");
    }

    fn ray_color(ray: &Ray,world: &HittableList) -> Rgb<u8> {
        // 增加了t的计算
        let mut rec = HitRecord{
            hit_point:Vec3::new(0.0,0.0,0.0),
            normal: Vec3::new(0.0,0.0,0.0),
            t: 0.0,
            front_face: true};
        let interval = Interval::new(0.0, f64::INFINITY);
        let flag = world.hit(&ray,&interval,& mut rec);
        if flag {
            let color = (rec.normal + Vec3{x:1.0,y:1.0,z:1.0}) * 0.5 * 256.0;
            color.to_rgb()
        }
        else{
            let unit_direction = ray.direction().normalize();
            let a = (unit_direction.get_y() + 1.0) * 0.5;
            let mut color = Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
            color = color * 256.0;
            color.to_rgb()
        }
    }
}