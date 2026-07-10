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
    let field_of_view = 90.0;
    let r = (std::f64::consts::PI / 4.0).cos();
    let camera = Camera::new(
        aspect_ration,
        width,
        samples_per_pixel,
        camera_max_depth,
        field_of_view,
    );
    // 定义材质
    let material_left = Arc::new(Lambertian {
        albedo: Vec3::new(0.0, 0.0, 1.0),
    });
    let material_right = Arc::new(Lambertian {
        albedo: Vec3::new(1.0, 0.0, 0.0),
    });
    // 创建世界
    let mut world = HittableList::new();
    let sphere_left = Sphere::new(Vec3::new(-r, -0.0, -1.0), r, material_left);
    let sphere_right = Sphere::new(Vec3::new(r, 0.0, -1.0), r, material_right);
    world.add(&sphere_left);
    world.add(&sphere_right);
    // 渲染图片
    camera.render(&world);
}
