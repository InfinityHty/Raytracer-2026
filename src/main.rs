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
use rand::{RngExt, rng};
use std::rc::Rc;
fn main() {
    // 创建相机
    let aspect_ration = 16.0 / 9.0;
    let width = 1200;
    let samples_per_pixel = 100;
    let camera_max_depth = 50;
    let field_of_view = 20.0;
    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
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
        defocus_angle,
        focus_dist,
    );
    // 定义材质
    let material_ground = Rc::new(Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    });
    // 创建世界
    let mut world = HittableList::new();
    let ground = Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));
    world.add(ground);
    let mut rng = rng();
    for a in -11..11 {
        for b in -11..11 {
            let decide_material = rng.random_range(0.0..1.0);
            let center = Vec3::new(
                a as f64 + 0.9 * rng.random_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.random_range(0.0..1.0),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length_squared().sqrt() > 0.9 {
                if decide_material < 0.8 {
                    // diffuse
                    let albedo = Vec3::new(
                        rng.random_range(0.0..1.0) * rng.random_range(0.0..1.0),
                        rng.random_range(0.0..1.0) * rng.random_range(0.0..1.0),
                        rng.random_range(0.0..1.0) * rng.random_range(0.0..1.0),
                    );
                    let material = Rc::new(Lambertian { albedo });
                    let sphere = Rc::new(Sphere::new(center, 0.2, material));
                    world.add(sphere);
                } else if decide_material < 0.95 {
                    // metal
                    let albedo = Vec3::new(
                        rng.random_range(0.5..1.0),
                        rng.random_range(0.5..1.0),
                        rng.random_range(0.5..1.0),
                    );
                    let fuzz = rng.random_range(0.0..0.5);
                    let material = Rc::new(Metal { albedo, fuzz });
                    let sphere = Rc::new(Sphere::new(center, 0.2, material));
                    world.add(sphere);
                } else {
                    // glass
                    let material = Rc::new(Dielectrics {
                        refractive_index: 1.5,
                    });
                    let sphere = Rc::new(Sphere::new(center, 0.2, material));
                    world.add(sphere);
                }
            }
        }
    }
    let material1 = Rc::new(Dielectrics {
        refractive_index: 1.5,
    });
    let sphere1 = Rc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));
    world.add(sphere1);
    let material2 = Rc::new(Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    });
    let sphere2 = Rc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));
    world.add(sphere2);
    let material3 = Rc::new(Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    let sphere3 = Rc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));
    world.add(sphere3);
    // 渲染图片
    camera.render(&world);
}
