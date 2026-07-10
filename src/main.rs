mod vec3;
use vec3::Vec3;
mod hittable;
mod ray;
use hittable::*;
mod hittable_list;
use hittable_list::*;
mod camera;
mod interval;
mod material;
use camera::*;
use material::*;
use std::sync::Arc;
fn main() {
    // 创建相机
    let aspect_ration = 16.0 / 9.0;
    let width = 400;
    let samples_per_pixel = 10;
    let camera_max_depth = 50;
    let camera = Camera::new(aspect_ration, width, samples_per_pixel, camera_max_depth);
    // 定义材质
    let material_ground = Arc::new(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    let material_center = Arc::new(Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });
    let material_left = Arc::new(Dielectrics {
        refractive_index: 1.50,
    });
    let material_right = Arc::new(Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });
    // 创建世界
    let mut world = HittableList::new();
    let sphere_0 = Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center);
    let sphere_1 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere_2 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere_3 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right);
    world.add(&sphere_0); // 中央球体
    world.add(&sphere_1); // 地面
    world.add(&sphere_2); // 左边的金属球
    world.add(&sphere_3); // 右边的金属球
    // 渲染图片
    camera.render(&world);
}
