mod vec3;
use vec3::Vec3;
mod hittable;
mod ray;
use hittable::*;
use ray::Ray;
mod hittable_list;
use hittable_list::*;
mod axis_aligned_bounding_boxes;
mod camera;
mod interval;
mod material;
mod perlin;
mod texture;
use crate::perlin::PerlinNoise;
use camera::*;
use image::ImageReader;
use material::*;
use rand::{RngExt, rng};
use std::path::Path;
use std::sync::Arc;
use texture::*;
fn main() {
    let mut box1 = HittableList::new();
    let ground = Arc::new(Lambertian {
        texture: Arc::new(SolidColor::new(Vec3::new(0.48, 0.83, 0.53))),
    });
    let box_per_side = 20;
    for i in 0..box_per_side {
        for j in 0..box_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let mut rng = rng();
            let y1 = rng.random_range(1.0..101.0);
            let z1 = z0 + w;
            let small_box = Arc::new(Cube::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            ));
            box1.add(small_box);
        }
    }
    let mut world = HittableList::new();
    let box1_size = box1.objects.len();
    world.add(Arc::new(BvhNode::new(&mut box1.objects, 0, box1_size)));
    //
    let light = Arc::new(Emissive {
        emit_color: Vec3::new(7.0, 7.0, 7.0),
    });
    let light_source = Arc::new(Quad::new(
        Vec3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    ));
    world.add(light_source);

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian {
        texture: Arc::new(SolidColor::new(Vec3::new(0.7, 0.3, 0.1))),
    });
    let sphere = Arc::new(MovingSphere::new(
        Ray::new(center1, center2 - center1, 0.0),
        50.0,
        sphere_material,
    ));
    world.add(sphere);

    let glass_sphere = Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectrics {
            refractive_index: 1.5,
        }),
    ));
    world.add(glass_sphere);
    let metal_sphere = Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal {
            albedo: Vec3::new(0.8, 0.8, 0.9),
            fuzz: 1.0,
        }),
    ));
    world.add(metal_sphere);

    let mut boundary = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectrics {
            refractive_index: 1.5,
        }),
    ));
    world.add(boundary.clone());
    let smoke = Arc::new(ConstantMedium::new(
        boundary,
        0.2,
        Arc::new(Isotropic::new(Arc::new(SolidColor::new(Vec3::new(
            0.2, 0.4, 0.9,
        ))))),
    ));
    world.add(smoke);
    boundary = Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectrics {
            refractive_index: 1.5,
        }),
    ));
    let atmosphere = Arc::new(ConstantMedium::new(
        boundary,
        0.0001,
        Arc::new(Isotropic::new(Arc::new(SolidColor::new(Vec3::new(
            1.0, 1.0, 1.0,
        ))))),
    ));
    world.add(atmosphere);

    let img_path = Path::new("texture_images/earthmap.jpg");
    let img = ImageReader::open(img_path)
        .expect("文件不存在")
        .decode()
        .expect("图片解码失败");
    let earth_material = Arc::new(Lambertian {
        texture: Arc::new(ImageTexture::new(img)),
    });
    world.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    )));

    let texture_perlin = Arc::new(Lambertian {
        texture: Arc::new(NoiseTexture {
            noise: PerlinNoise::new(),
            scale: 0.2,
        }),
    });
    let noise_sphere = Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        texture_perlin,
    ));
    world.add(noise_sphere);

    let mut box2 = HittableList::new();
    let white = Arc::new(Lambertian {
        texture: Arc::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))),
    });
    let ns = 1000;
    let mut rng = rng();
    for _i in 0..ns {
        box2.add(Arc::new(Sphere::new(
            Vec3::new(
                rng.random_range(0.0..165.0),
                rng.random_range(0.0..165.0),
                rng.random_range(0.0..165.0),
            ),
            10.0,
            white.clone(),
        )));
    }
    let box2_size = box2.objects.len();
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new(&mut box2.objects, 0, box2_size)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let aspect_ration = 1.0;
    let width = 800;
    let samples_per_pixel = 250;
    let camera_max_depth = 40;
    let field_of_view = 40.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let look_from = Vec3::new(478.0, 278.0, -600.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let background = Vec3::new(0.0, 0.0, 0.0);
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
        background,
    );
    let mut bvh_world = HittableList::new();
    let cnt = world.objects.len();
    bvh_world.add(Arc::new(BvhNode::new(&mut world.objects, 0, cnt)));
    // 渲染图片
    Arc::new(camera).render(Arc::new(bvh_world));
}
// #[allow(dead_code)]
// fn cornell_smoke() {
//     let mut world = HittableList::new();
//     let red = Rc::new(Lambertian {
//         texture: Rc::new(SolidColor::new(Vec3::new(0.65, 0.05, 0.05))),
//     });
//     let white = Rc::new(Lambertian {
//         texture: Rc::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))),
//     });
//     let green = Rc::new(Lambertian {
//         texture: Rc::new(SolidColor::new(Vec3::new(0.12, 0.45, 0.15))),
//     });
//     let light = Rc::new(Emissive {
//         emit_color: Vec3::new(7.0, 7.0, 7.0),
//     });
//
//     let wall0 = Rc::new(Quad::new(
//         Vec3::new(555.0, 0.0, 0.0),
//         Vec3::new(0.0, 555.0, 0.0),
//         Vec3::new(0.0, 0.0, 555.0),
//         green,
//     ));
//     let wall1 = Rc::new(Quad::new(
//         Vec3::new(0.0, 0.0, 0.0),
//         Vec3::new(0.0, 555.0, 0.0),
//         Vec3::new(0.0, 0.0, 555.0),
//         red,
//     ));
//     let light_source = Rc::new(Quad::new(
//         Vec3::new(113.0, 554.0, 127.0),
//         Vec3::new(330.0, 0.0, 0.0),
//         Vec3::new(0.0, 0.0, 305.0),
//         light,
//     ));
//     let wall3 = Rc::new(Quad::new(
//         Vec3::new(0.0, 0.0, 0.0),
//         Vec3::new(555.0, 0.0, 0.0),
//         Vec3::new(0.0, 0.0, 555.0),
//         white.clone(),
//     ));
//     let wall4 = Rc::new(Quad::new(
//         Vec3::new(555.0, 555.0, 555.0),
//         Vec3::new(-555.0, 0.0, 0.0),
//         Vec3::new(0.0, 0.0, -555.0),
//         white.clone(),
//     ));
//     let wall5 = Rc::new(Quad::new(
//         Vec3::new(0.0, 0.0, 555.0),
//         Vec3::new(555.0, 0.0, 0.0),
//         Vec3::new(0.0, 555.0, 0.0),
//         white.clone(),
//     ));
//     world.add(wall0);
//     world.add(wall1);
//     world.add(light_source);
//     world.add(wall3);
//     world.add(wall4);
//     world.add(wall5);
//     let box1 = Rc::new(Cube::new(
//         Vec3::new(0.0, 0.0, 0.0),
//         Vec3::new(165.0, 330.0, 165.0),
//         white.clone(),
//     ));
//     let box2 = Rc::new(Cube::new(
//         Vec3::new(0.0, 0.0, 0.0),
//         Vec3::new(165.0, 165.0, 165.0),
//         white.clone(),
//     ));
//
//     let rotated_box1 = Rc::new(RotateY::new(box1, 15.0));
//     let rotated_box2 = Rc::new(RotateY::new(box2, -18.0));
//     let moved_box1 = Rc::new(Translate::new(rotated_box1, Vec3::new(265.0, 0.0, 295.0)));
//     let moved_box2 = Rc::new(Translate::new(rotated_box2, Vec3::new(130.0, 0.0, 65.0)));
//     let smoke_box1 = Rc::new(ConstantMedium::new(
//         moved_box1,
//         0.01,
//         Rc::new(Isotropic {
//             texture: Rc::new(SolidColor::new(Vec3::new(0.0, 0.0, 0.0))),
//         }),
//     ));
//     let smoke_box2 = Rc::new(ConstantMedium::new(
//         moved_box2,
//         0.01,
//         Rc::new(Isotropic {
//             texture: Rc::new(SolidColor::new(Vec3::new(1.0, 1.0, 1.0))),
//         }),
//     ));
//     world.add(smoke_box1);
//     world.add(smoke_box2);
//
//     let aspect_ration = 1.0;
//     let width = 600;
//     let samples_per_pixel = 200;
//     let camera_max_depth = 50;
//     let field_of_view = 40.0;
//     let defocus_angle = 0.0;
//     let focus_dist = 10.0;
//     let look_from = Vec3::new(278.0, 278.0, -800.0);
//     let look_at = Vec3::new(278.0, 278.0, 0.0);
//     let view_up = Vec3::new(0.0, 1.0, 0.0);
//     let background = Vec3::new(0.0, 0.0, 0.0);
//     let camera = Camera::new(
//         aspect_ration,
//         width,
//         samples_per_pixel,
//         camera_max_depth,
//         field_of_view,
//         look_from,
//         look_at,
//         view_up,
//         defocus_angle,
//         focus_dist,
//         background,
//     );
//     let mut bvh_world = HittableList::new();
//     let cnt = world.objects.len();
//     bvh_world.add(Rc::new(BvhNode::new(&mut world.objects, 0, cnt)));
//     // 渲染图片
//     camera.render(&world);
// }
// #[allow(dead_code)]
// fn cornell_box() {
//     let mut world = HittableList::new();
//     let red = Rc::new(Lambertian {
//         texture: Rc::new(SolidColor::new(Vec3::new(0.65, 0.05, 0.05))),
//     });
//     let white = Rc::new(Lambertian {
//         texture: Rc::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))),
//     });
//     let green = Rc::new(Lambertian {
//         texture: Rc::new(SolidColor::new(Vec3::new(0.12, 0.45, 0.15))),
//     });
//     let light = Rc::new(Emissive {
//         emit_color: Vec3::new(15.0, 15.0, 15.0),
//     });
//
//     let wall0 = Rc::new(Quad::new(
//         Vec3::new(555.0, 0.0, 0.0),
//         Vec3::new(0.0, 555.0, 0.0),
//         Vec3::new(0.0, 0.0, 555.0),
//         green,
//     ));
//     let wall1 = Rc::new(Quad::new(
//         Vec3::new(0.0, 0.0, 0.0),
//         Vec3::new(0.0, 555.0, 0.0),
//         Vec3::new(0.0, 0.0, 555.0),
//         red,
//     ));
//     let light_source = Rc::new(Quad::new(
//         Vec3::new(343.0, 554.0, 332.0),
//         Vec3::new(-130.0, 0.0, 0.0),
//         Vec3::new(0.0, 0.0, -105.0),
//         light,
//     ));
//     let wall3 = Rc::new(Quad::new(
//         Vec3::new(0.0, 0.0, 0.0),
//         Vec3::new(555.0, 0.0, 0.0),
//         Vec3::new(0.0, 0.0, 555.0),
//         white.clone(),
//     ));
//     let wall4 = Rc::new(Quad::new(
//         Vec3::new(555.0, 555.0, 555.0),
//         Vec3::new(-555.0, 0.0, 0.0),
//         Vec3::new(0.0, 0.0, -555.0),
//         white.clone(),
//     ));
//     let wall5 = Rc::new(Quad::new(
//         Vec3::new(0.0, 0.0, 555.0),
//         Vec3::new(555.0, 0.0, 0.0),
//         Vec3::new(0.0, 555.0, 0.0),
//         white.clone(),
//     ));
//     world.add(wall0);
//     world.add(wall1);
//     world.add(light_source);
//     world.add(wall3);
//     world.add(wall4);
//     world.add(wall5);
//     let box1 = Rc::new(Cube::new(
//         Vec3::new(0.0, 0.0, 0.0),
//         Vec3::new(165.0, 330.0, 165.0),
//         white.clone(),
//     ));
//     let box2 = Rc::new(Cube::new(
//         Vec3::new(0.0, 0.0, 0.0),
//         Vec3::new(165.0, 165.0, 165.0),
//         white.clone(),
//     ));
//     // let box1 = Rc::new(Cube::new(Vec3::new(265.0,0.0,295.0),Vec3::new(430.0,330.0,460.0),white.clone()));
//     // let box2 = Rc::new(Cube::new(Vec3::new(130.0,0.0,65.0),Vec3::new(295.0,165.0,230.0),white.clone()));
//     let rotated_box1 = Rc::new(RotateY::new(box1, 15.0));
//     let rotated_box2 = Rc::new(RotateY::new(box2, -18.0));
//     let moved_box1 = Rc::new(Translate::new(rotated_box1, Vec3::new(265.0, 0.0, 295.0)));
//     let moved_box2 = Rc::new(Translate::new(rotated_box2, Vec3::new(130.0, 0.0, 65.0)));
//     // world.add_box(Vec3::new(130.0, 0.0, 65.0),Vec3::new(295.0, 165.0, 230.0),white.clone());
//     // world.add_box(Vec3::new(265.0,0.0,295.0),Vec3::new(430.0,330.0,460.0),white.clone());
//     world.add(moved_box1);
//     world.add(moved_box2);
//
//     let aspect_ration = 1.0;
//     let width = 600;
//     let samples_per_pixel = 200;
//     let camera_max_depth = 50;
//     let field_of_view = 40.0;
//     let defocus_angle = 0.0;
//     let focus_dist = 10.0;
//     let look_from = Vec3::new(278.0, 278.0, -800.0);
//     let look_at = Vec3::new(278.0, 278.0, 0.0);
//     let view_up = Vec3::new(0.0, 1.0, 0.0);
//     let background = Vec3::new(0.0, 0.0, 0.0);
//     let camera = Camera::new(
//         aspect_ration,
//         width,
//         samples_per_pixel,
//         camera_max_depth,
//         field_of_view,
//         look_from,
//         look_at,
//         view_up,
//         defocus_angle,
//         focus_dist,
//         background,
//     );
//     let mut bvh_world = HittableList::new();
//     let cnt = world.objects.len();
//     bvh_world.add(Rc::new(BvhNode::new(&mut world.objects, 0, cnt)));
//     // 渲染图片
//     camera.render(&world);
// }
//
// #[allow(dead_code)]
// fn simple_light() {
//     let aspect_ration = 16.0 / 9.0;
//     let width = 400;
//     let samples_per_pixel = 100;
//     let camera_max_depth = 50;
//     let field_of_view = 20.0;
//     let defocus_angle = 0.0;
//     let focus_dist = 10.0;
//     let look_from = Vec3::new(26.0, 3.0, 6.0);
//     let look_at = Vec3::new(0.0, 2.0, 0.0);
//     let view_up = Vec3::new(0.0, 1.0, 0.0);
//     let background = Vec3::new(0.0, 0.0, 0.0);
//     let camera = Camera::new(
//         aspect_ration,
//         width,
//         samples_per_pixel,
//         camera_max_depth,
//         field_of_view,
//         look_from,
//         look_at,
//         view_up,
//         defocus_angle,
//         focus_dist,
//         background,
//     );
//     let mut world = HittableList::new();
//     let perlin_texture = Rc::new(Lambertian {
//         texture: Rc::new(NoiseTexture {
//             noise: PerlinNoise::new(),
//             scale: 4.0,
//         }),
//     });
//     let sphere = Rc::new(Sphere::new(
//         Vec3::new(0.0, 2.0, 0.0),
//         2.0,
//         perlin_texture.clone(),
//     ));
//     let ground = Rc::new(Sphere::new(
//         Vec3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         perlin_texture,
//     ));
//     world.add(ground);
//     world.add(sphere);
//     let light_material = Rc::new(Emissive {
//         emit_color: Vec3::new(4.0, 4.0, 4.0), // 超乎寻常的亮度
//     });
//     let light_rectangle = Rc::new(Quad::new(
//         Vec3::new(3.0, 1.0, -2.0),
//         Vec3::new(2.0, 0.0, 0.0),
//         Vec3::new(0.0, 2.0, 0.0),
//         light_material.clone(),
//     ));
//     let light_sphere = Rc::new(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 2.0, light_material));
//     world.add(light_rectangle);
//     world.add(light_sphere);
//     camera.render(&world);
// }
// #[allow(dead_code)]
// fn quads() {
//     let aspect_ration = 1.0;
//     let width = 400;
//     let samples_per_pixel = 100;
//     let camera_max_depth = 50;
//     let field_of_view = 80.0;
//     let defocus_angle = 0.0;
//     let focus_dist = 10.0;
//     let look_from = Vec3::new(0.0, 0.0, 9.0);
//     let look_at = Vec3::new(0.0, 0.0, 0.0);
//     let view_up = Vec3::new(0.0, 1.0, 0.0);
//     let background = Vec3::new(0.7, 0.8, 1.0);
//     let camera = Camera::new(
//         aspect_ration,
//         width,
//         samples_per_pixel,
//         camera_max_depth,
//         field_of_view,
//         look_from,
//         look_at,
//         view_up,
//         defocus_angle,
//         focus_dist,
//         background,
//     );
//     let mut world = HittableList::new();
//     let left_red = Rc::new(Lambertian {
//         texture: Rc::new(SolidColor::new(Vec3::new(1.0, 0.2, 0.2))),
//     });
//     let back_green = Rc::new(Lambertian {
//         texture: Rc::new(SolidColor::new(Vec3::new(0.2, 1.0, 0.2))),
//     });
//     let right_blue = Rc::new(Lambertian {
//         texture: Rc::new(SolidColor::new(Vec3::new(0.2, 0.2, 1.0))),
//     });
//     let upper_orange = Rc::new(Lambertian {
//         texture: Rc::new(SolidColor::new(Vec3::new(1.0, 0.5, 0.0))),
//     });
//     let lower_teal = Rc::new(Lambertian {
//         texture: Rc::new(SolidColor::new(Vec3::new(0.2, 0.8, 0.8))),
//     });
//
//     let quad0 = Rc::new(Quad::new(
//         Vec3::new(-3.0, -2.0, 5.0),
//         Vec3::new(0.0, 0.0, -4.0),
//         Vec3::new(0.0, 4.0, 0.0),
//         left_red,
//     ));
//     let quad1 = Rc::new(Quad::new(
//         Vec3::new(-2.0, -2.0, 0.0),
//         Vec3::new(4.0, 0.0, 0.0),
//         Vec3::new(0.0, 4.0, 0.0),
//         back_green,
//     ));
//     let quad2 = Rc::new(Quad::new(
//         Vec3::new(3.0, -2.0, 1.0),
//         Vec3::new(0.0, 0.0, 4.0),
//         Vec3::new(0.0, 4.0, 0.0),
//         right_blue,
//     ));
//     let quad3 = Rc::new(Quad::new(
//         Vec3::new(-2.0, 3.0, 1.0),
//         Vec3::new(4.0, 0.0, 0.0),
//         Vec3::new(0.0, 0.0, 4.0),
//         upper_orange,
//     ));
//     let quad4 = Rc::new(Quad::new(
//         Vec3::new(-2.0, -3.0, 5.0),
//         Vec3::new(4.0, 0.0, 0.0),
//         Vec3::new(0.0, 0.0, -4.0),
//         lower_teal,
//     ));
//     world.add(quad0);
//     world.add(quad1);
//     world.add(quad2);
//     world.add(quad3);
//     world.add(quad4);
//     camera.render(&world);
// }
// #[allow(dead_code)]
// fn bouncing_spheres() {
//     // 创建相机
//     let aspect_ration = 16.0 / 9.0;
//     let width = 400;
//     let samples_per_pixel = 100;
//     let camera_max_depth = 50;
//     let field_of_view = 20.0;
//     let defocus_angle = 0.6;
//     let focus_dist = 10.0;
//
//     let look_from = Vec3::new(13.0, 2.0, 3.0);
//     let look_at = Vec3::new(0.0, 0.0, 0.0);
//     let view_up = Vec3::new(0.0, 1.0, 0.0);
//     let background = Vec3::new(0.7, 0.8, 1.0);
//     let camera = Camera::new(
//         aspect_ration,
//         width,
//         samples_per_pixel,
//         camera_max_depth,
//         field_of_view,
//         look_from,
//         look_at,
//         view_up,
//         defocus_angle,
//         focus_dist,
//         background,
//     );
//     // 定义材质
//     let texture_odd = Rc::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1)));
//     let texture_even = Rc::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9)));
//     let material_ground = Rc::new(Lambertian {
//         texture: Rc::new(CheckeredTexture::new(texture_odd, texture_even, 0.32)),
//     });
//     // 创建世界
//     let mut world = HittableList::new();
//     let ground = Rc::new(Sphere::new(
//         Vec3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         material_ground,
//     ));
//     world.add(ground);
//     let mut rng = rng();
//     for a in -11..11 {
//         for b in -11..11 {
//             let decide_material = rng.random_range(0.0..1.0);
//             let center = Vec3::new(
//                 a as f64 + 0.9 * rng.random_range(0.0..1.0),
//                 0.2,
//                 b as f64 + 0.9 * rng.random_range(0.0..1.0),
//             );
//             if (center - Vec3::new(4.0, 0.2, 0.0)).length_squared().sqrt() > 0.9 {
//                 if decide_material < 0.8 {
//                     // diffuse
//                     let albedo = Vec3::new(
//                         rng.random_range(0.0..1.0) * rng.random_range(0.0..1.0),
//                         rng.random_range(0.0..1.0) * rng.random_range(0.0..1.0),
//                         rng.random_range(0.0..1.0) * rng.random_range(0.0..1.0),
//                     );
//                     let material = Rc::new(Lambertian {
//                         texture: Rc::new(SolidColor::new(albedo)),
//                     });
//                     let route = Vec3::new(0.0, rng.random_range(0.0..0.5), 0.0);
//                     let moving_route = Ray::new(center, route, 0.0);
//                     let moving_sphere = Rc::new(MovingSphere::new(moving_route, 0.2, material));
//                     world.add(moving_sphere);
//                 } else if decide_material < 0.95 {
//                     // metal
//                     let albedo = Vec3::new(
//                         rng.random_range(0.5..1.0),
//                         rng.random_range(0.5..1.0),
//                         rng.random_range(0.5..1.0),
//                     );
//                     let fuzz = rng.random_range(0.0..0.5);
//                     let material = Rc::new(Metal { albedo, fuzz });
//                     let sphere = Rc::new(Sphere::new(center, 0.2, material));
//                     world.add(sphere);
//                 } else {
//                     // glass
//                     let material = Rc::new(Dielectrics {
//                         refractive_index: 1.5,
//                     });
//                     let sphere = Rc::new(Sphere::new(center, 0.2, material));
//                     world.add(sphere);
//                 }
//             }
//         }
//     }
//     let material1 = Rc::new(Dielectrics {
//         refractive_index: 1.5,
//     });
//     let sphere1 = Rc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));
//     world.add(sphere1);
//     let material2 = Rc::new(Lambertian {
//         texture: Rc::new(SolidColor::new(Vec3::new(0.4, 0.2, 0.1))),
//     });
//     let sphere2 = Rc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));
//     world.add(sphere2);
//     let material3 = Rc::new(Metal {
//         albedo: Vec3::new(0.7, 0.6, 0.5),
//         fuzz: 0.0,
//     });
//     let sphere3 = Rc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));
//     world.add(sphere3);
//     let mut bvh_world = HittableList::new();
//     let cnt = world.objects.len();
//     bvh_world.add(Rc::new(BvhNode::new(&mut world.objects, 0, cnt)));
//     // 渲染图片
//     camera.render(&world);
// }
// #[allow(dead_code)]
// fn checker_spheres() {
//     let aspect_ration = 16.0 / 9.0;
//     let width = 400;
//     let samples_per_pixel = 100;
//     let camera_max_depth = 50;
//     let field_of_view = 20.0;
//     let defocus_angle = 0.0;
//     let focus_dist = 10.0;
//     let look_from = Vec3::new(13.0, 2.0, 3.0);
//     let look_at = Vec3::new(0.0, 0.0, 0.0);
//     let view_up = Vec3::new(0.0, 1.0, 0.0);
//     let background = Vec3::new(0.7, 0.8, 1.0);
//     let camera = Camera::new(
//         aspect_ration,
//         width,
//         samples_per_pixel,
//         camera_max_depth,
//         field_of_view,
//         look_from,
//         look_at,
//         view_up,
//         defocus_angle,
//         focus_dist,
//         background,
//     );
//     let mut world = HittableList::new();
//     let texture_odd = Rc::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1)));
//     let texture_even = Rc::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9)));
//     let checker1 = Rc::new(Lambertian {
//         texture: Rc::new(CheckeredTexture::new(texture_odd, texture_even, 0.32)),
//     });
//     let checker2 = checker1.clone();
//     let sphere1 = Rc::new(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, checker1));
//     let sphere2 = Rc::new(Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, checker2));
//     world.add(sphere1);
//     world.add(sphere2);
//     camera.render(&world);
// }
// #[allow(dead_code)]
// fn earth() {
//     let img_path = Path::new("texture_images/earthmap.jpg");
//     let img = ImageReader::open(img_path)
//         .expect("文件不存在")
//         .decode()
//         .expect("图片解码失败");
//
//     let aspect_ration = 16.0 / 9.0;
//     let width = img.width();
//     let samples_per_pixel = 100;
//     let camera_max_depth = 50;
//     let field_of_view = 20.0;
//     let defocus_angle = 0.0;
//     let focus_dist = 10.0;
//     let look_from = Vec3::new(0.0, 0.0, 12.0);
//     let look_at = Vec3::new(0.0, 0.0, 0.0);
//     let view_up = Vec3::new(0.0, 1.0, 0.0);
//     let background = Vec3::new(0.7, 0.8, 1.00);
//     let camera = Camera::new(
//         aspect_ration,
//         width,
//         samples_per_pixel,
//         camera_max_depth,
//         field_of_view,
//         look_from,
//         look_at,
//         view_up,
//         defocus_angle,
//         focus_dist,
//         background,
//     );
//     let mut world = HittableList::new();
//     let earth_texture = Rc::new(ImageTexture::new(img));
//     let earth_surface = Rc::new(Lambertian {
//         texture: earth_texture,
//     });
//     let earth = Rc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
//     world.add(earth);
//     camera.render(&world);
// }
// #[allow(dead_code)]
// fn perlin_sphere() {
//     let aspect_ration = 16.0 / 9.0;
//     let width = 400;
//     let samples_per_pixel = 100;
//     let camera_max_depth = 50;
//     let field_of_view = 20.0;
//     let defocus_angle = 0.0;
//     let focus_dist = 10.0;
//     let look_from = Vec3::new(13.0, 2.0, 3.0);
//     let look_at = Vec3::new(0.0, 0.0, 0.0);
//     let view_up = Vec3::new(0.0, 1.0, 0.0);
//     let background = Vec3::new(0.7, 0.8, 1.0);
//     let camera = Camera::new(
//         aspect_ration,
//         width,
//         samples_per_pixel,
//         camera_max_depth,
//         field_of_view,
//         look_from,
//         look_at,
//         view_up,
//         defocus_angle,
//         focus_dist,
//         background,
//     );
//     let mut world = HittableList::new();
//     let texture_perlin = Rc::new(Lambertian {
//         texture: Rc::new(NoiseTexture {
//             noise: PerlinNoise::new(),
//             scale: 4.0,
//         }),
//     });
//     let ground = Rc::new(Sphere::new(
//         Vec3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         texture_perlin.clone(),
//     ));
//     let sphere = Rc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, texture_perlin));
//     world.add(ground);
//     world.add(sphere);
//     camera.render(&world);
// }
