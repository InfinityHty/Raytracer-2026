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
    let samples_per_pixel = 100;
    let camera_max_depth = 50;
    let field_of_view = 20.0;
    let defoucs_angle = 10.0;
    let focus_dist = 3.4;

    let look_from = Vec3::new(-2.0, 2.0, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        aspect_ration,
        width,
        samples_per_pixel,
        camera_max_depth,
        field_of_view,
        look_from,
        look_at,
        view_up,
        defoucs_angle,
        focus_dist,
    );
    // 定义材质
    let material_ground = Arc::new(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    let material_center = Arc::new(Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });
    let material_left = Arc::new(Dielectrics {
        refractive_index: 1.5,
    });
    let material_right = Arc::new(Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });
    let material_bubble = Arc::new(Dielectrics {
        refractive_index: 1.0 / 1.5,
    });
    // 创建世界
    let mut world = HittableList::new();
    let sphere0 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center);
    let sphere2 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere3 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right);
    let bubble = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, material_bubble);
    world.add(&sphere0);
    world.add(&sphere1);
    world.add(&sphere2);
    world.add(&sphere3);
    world.add(&bubble);
    // 渲染图片
    camera.render(&world);
}
