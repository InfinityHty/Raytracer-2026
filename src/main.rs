use console::style; // 给控制台打印的文字加颜色、加粗等样式
use image::{ImageBuffer, Rgb, RgbImage}; // Rust最主流的图像处理库 创建画布 逐像素绘制光线追踪结果 导出文件图片
use indicatif::ProgressBar; // 进度条可视化
mod vec3;
use vec3::Vec3;
mod ray;
use ray::Ray;
fn main() {
    // 创建保存路径
    let path = std::path::Path::new("output/book1/image3.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    // 设置eye point viewport等等参数
    let aspect_ration = 16.0 / 9.0;
    let eye_point: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let f = 1.0;

    let width = 400;
    let height = (width as f64 / aspect_ration) as u32;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width as f64 / height as f64);

    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);

    let pixel_u = viewport_u / width as f64;
    let pixel_v = viewport_v / height as f64;

    let upper_left = eye_point - Vec3::new(0.0, 0.0, f) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_00 = upper_left + pixel_u * 0.5 + pixel_v * 0.5;
    let center_of_sphere = Vec3::new(0.0, 0.0, -f);
    // different from the book, we use image crate to create a .png image rather than outputting .ppm file, which is not widely used.
    // anyway, you may output any image format you like.
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in 0..height {
        for i in 0..width {
            let pixel_ij = pixel_00 + pixel_u * i as f64 + pixel_v * j as f64;
            // println!("{}",pixel_ij.get_y());
            let ray_direction = pixel_ij - eye_point;
            let ray = Ray::new(eye_point, ray_direction);
            let pixel = img.get_pixel_mut(i, j);

            *pixel = ray_color(&ray, &center_of_sphere);
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
fn ray_color(ray: &Ray, center: &Vec3) -> Rgb<u8> {
    if hit_sphere(*center, 0.5, ray) {
        let color = vec3::Vec3::new(256.0, 0.0, 0.0);
        color.to_rgb()
    } else {
        let unit_direction = ray.direction().normalize();
        let a = (unit_direction.get_y() + 1.0) * 0.5;
        let mut color = Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
        color = color * 256.0;
        color.to_rgb()
    }
}
fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let a = ray.direction().length_squared();
    let b = ray.direction() * (center - ray.ori()) * -2.0;
    let c = (center - ray.ori()).length_squared() - radius * radius;
    if b * b - 4.0 * a * c < 0.0 {
        return false;
    }
    true
}
