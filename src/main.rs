mod vec3;
use vec3::Vec3;
mod hittable;
mod ray;
use hittable::*;
mod hittable_list;
use hittable_list::*;
mod camera;
mod interval;
use camera::*;
fn main() {
    // 创建相机
    let aspect_ration = 16.0 / 9.0;
    let width = 400;
    let samples_per_pixel = 10;
    let camera_max_depth = 50;
    let camera = Camera::new(aspect_ration, width, samples_per_pixel, camera_max_depth);
    // 创建世界
    let mut world = HittableList::new();
    let sphere_0 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere_1 = Sphere::new(
        Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        100.0,
    );
    world.add(&sphere_0); // 中央球体
    world.add(&sphere_1); // 地面
    // 渲染图片
    camera.render(&world);
}
