use crate::hittable::*;
use crate::hittable_list::*;
use crate::interval::*;
use crate::ray::*;
use crate::vec3::*;
use console::style; // 给控制台打印的文字加颜色、加粗等样式
use image::{ImageBuffer, RgbImage}; // Rust最主流的图像处理库 创建画布 逐像素绘制光线追踪结果 导出文件图片
use indicatif::ProgressBar; // 进度条可视化
use rand::{RngExt, rng};
pub struct Camera {
    aspect_ration: f64,
    width: u32,
    #[allow(dead_code)]
    samples_per_pixel: u32,
    max_depth: u32,
}
impl Camera {
    pub fn new(
        aspect_ration: f64,
        width: u32,
        samples_per_pixel: u32,
        camera_max_depth: u32,
    ) -> Self {
        Self {
            aspect_ration,
            width,
            samples_per_pixel,
            max_depth: camera_max_depth,
        }
    }
    pub fn render(&self, world: &HittableList) {
        // 保存路径
        let path = std::path::Path::new("output/book1/image10.png");
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
        // 相机内参
        let eye_point: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let f = 1.0;

        let height = (self.width as f64 / self.aspect_ration) as u32;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.width as f64 / height as f64);

        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);

        let pixel_u = viewport_u / self.width as f64;
        let pixel_v = viewport_v / height as f64;

        let upper_left = eye_point - Vec3::new(0.0, 0.0, f) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00 = upper_left + pixel_u * 0.5 + pixel_v * 0.5;

        let mut img: RgbImage = ImageBuffer::new(self.width, height);

        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((height * self.width) as u64)
        };
        // 逐像素渲染
        for j in 0..height {
            for i in 0..self.width {
                let pixel_ij = pixel_00 + pixel_u * i as f64 + pixel_v * j as f64;
                // let ray_direction = pixel_ij - eye_point;
                // let ray = Ray::new(eye_point, ray_direction);
                let pixel = img.get_pixel_mut(i, j);
                // let pixel_color = Camera::ray_color(&ray, world);
                // *pixel = (pixel_color * 256.0).to_rgb();

                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _sample_times in 0..self.samples_per_pixel {
                    let ray = Camera::get_ray(&pixel_ij, &eye_point, pixel_u.x, pixel_v.y);
                    pixel_color = pixel_color + Camera::ray_color(&ray, world, self.max_depth);
                }
                pixel_color = pixel_color / self.samples_per_pixel as f64;
                let color_interval = Interval::new(0.0, 1.0);
                pixel_color.x = color_interval.clamp(pixel_color.x);
                pixel_color.y = color_interval.clamp(pixel_color.y);
                pixel_color.z = color_interval.clamp(pixel_color.z);
                pixel_color = pixel_color * 256.0;
                *pixel = pixel_color.to_rgb();
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

    fn ray_color(ray: &Ray, world: &HittableList, depth: u32) -> Vec3 {
        if depth == 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        // 防止过度递归
        let mut rec = HitRecord {
            hit_point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
        };
        let interval = Interval::new(0.001, f64::INFINITY);
        let flag = world.hit(ray, &interval, &mut rec);
        if flag {
            // 余弦分布
            let reflection_direction = rec.normal + Vec3::generate_rand_norm(-1.0, 1.0);
            let reflection_ray = Ray::new(rec.hit_point, reflection_direction);
            Camera::ray_color(&reflection_ray, world, depth - 1) * 0.5
        } else {
            let unit_direction = ray.direction().normalize();
            let a = (unit_direction.get_y() + 1.0) * 0.5;
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
        }
    }
    #[allow(dead_code)]
    fn get_ray(pixel_center: &Vec3, eye_point: &Vec3, delta_u: f64, delta_v: f64) -> Ray {
        // 一定范围内随机采样
        let pixel_sample = Vec3::new(
            pixel_center.x + delta_u * Camera::random(0.5),
            pixel_center.y + delta_v * Camera::random(0.5),
            pixel_center.z,
        );
        let origin = Vec3::new(eye_point.x, eye_point.y, eye_point.z);
        let direction = pixel_sample - origin;
        Ray::new(origin, direction)
    }
    fn random(range: f64) -> f64 {
        let mut rng = rng();
        rng.random_range(-range..range)
    }
}
