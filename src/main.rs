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
use std::rc::Rc;
use texture::*;
fn main() {
    let mut world = HittableList::new();
    let red = Rc::new(Lambertian {
        texture: Rc::new(SolidColor::new(Vec3::new(0.65, 0.05, 0.05))),
    });
    let white = Rc::new(Lambertian {
        texture: Rc::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))),
    });
    let green = Rc::new(Lambertian {
        texture: Rc::new(SolidColor::new(Vec3::new(0.12, 0.45, 0.15))),
    });
    let light = Rc::new(Emissive {
        emit_color: Vec3::new(7.0, 7.0, 7.0),
    });

    let wall0 = Rc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    let wall1 = Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    let light_source = Rc::new(Quad::new(
        Vec3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    ));
    let wall3 = Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    let wall4 = Rc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    let wall5 = Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));
    world.add(wall0);
    world.add(wall1);
    world.add(light_source);
    world.add(wall3);
    world.add(wall4);
    world.add(wall5);
    let box1 = Rc::new(Cube::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box2 = Rc::new(Cube::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));

    let rotated_box1 = Rc::new(RotateY::new(box1, 15.0));
    let rotated_box2 = Rc::new(RotateY::new(box2, -18.0));
    let moved_box1 = Rc::new(Translate::new(rotated_box1, Vec3::new(265.0, 0.0, 295.0)));
    let moved_box2 = Rc::new(Translate::new(rotated_box2, Vec3::new(130.0, 0.0, 65.0)));
    let smoke_box1 = Rc::new(ConstantMedium::new(
        moved_box1,
        0.01,
        Rc::new(Isotropic {
            texture: Rc::new(SolidColor::new(Vec3::new(0.0, 0.0, 0.0))),
        }),
    ));
    let smoke_box2 = Rc::new(ConstantMedium::new(
        moved_box2,
        0.01,
        Rc::new(Isotropic {
            texture: Rc::new(SolidColor::new(Vec3::new(1.0, 1.0, 1.0))),
        }),
    ));
    world.add(smoke_box1);
    world.add(smoke_box2);

    let aspect_ration = 1.0;
    let width = 600;
    let samples_per_pixel = 200;
    let camera_max_depth = 50;
    let field_of_view = 40.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let look_from = Vec3::new(278.0, 278.0, -800.0);
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
    bvh_world.add(Rc::new(BvhNode::new(&mut world.objects, 0, cnt)));
    // 渲染图片
    camera.render(&world);
}
#[allow(dead_code)]
fn cornell_box() {
    let mut world = HittableList::new();
    let red = Rc::new(Lambertian {
        texture: Rc::new(SolidColor::new(Vec3::new(0.65, 0.05, 0.05))),
    });
    let white = Rc::new(Lambertian {
        texture: Rc::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))),
    });
    let green = Rc::new(Lambertian {
        texture: Rc::new(SolidColor::new(Vec3::new(0.12, 0.45, 0.15))),
    });
    let light = Rc::new(Emissive {
        emit_color: Vec3::new(15.0, 15.0, 15.0),
    });

    let wall0 = Rc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    let wall1 = Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    let light_source = Rc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));
    let wall3 = Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    let wall4 = Rc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    let wall5 = Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));
    world.add(wall0);
    world.add(wall1);
    world.add(light_source);
    world.add(wall3);
    world.add(wall4);
    world.add(wall5);
    let box1 = Rc::new(Cube::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box2 = Rc::new(Cube::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    // let box1 = Rc::new(Cube::new(Vec3::new(265.0,0.0,295.0),Vec3::new(430.0,330.0,460.0),white.clone()));
    // let box2 = Rc::new(Cube::new(Vec3::new(130.0,0.0,65.0),Vec3::new(295.0,165.0,230.0),white.clone()));
    let rotated_box1 = Rc::new(RotateY::new(box1, 15.0));
    let rotated_box2 = Rc::new(RotateY::new(box2, -18.0));
    let moved_box1 = Rc::new(Translate::new(rotated_box1, Vec3::new(265.0, 0.0, 295.0)));
    let moved_box2 = Rc::new(Translate::new(rotated_box2, Vec3::new(130.0, 0.0, 65.0)));
    // world.add_box(Vec3::new(130.0, 0.0, 65.0),Vec3::new(295.0, 165.0, 230.0),white.clone());
    // world.add_box(Vec3::new(265.0,0.0,295.0),Vec3::new(430.0,330.0,460.0),white.clone());
    world.add(moved_box1);
    world.add(moved_box2);

    let aspect_ration = 1.0;
    let width = 600;
    let samples_per_pixel = 200;
    let camera_max_depth = 50;
    let field_of_view = 40.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let look_from = Vec3::new(278.0, 278.0, -800.0);
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
    bvh_world.add(Rc::new(BvhNode::new(&mut world.objects, 0, cnt)));
    // 渲染图片
    camera.render(&world);
}

#[allow(dead_code)]
fn simple_light() {
    let aspect_ration = 16.0 / 9.0;
    let width = 400;
    let samples_per_pixel = 100;
    let camera_max_depth = 50;
    let field_of_view = 20.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let look_from = Vec3::new(26.0, 3.0, 6.0);
    let look_at = Vec3::new(0.0, 2.0, 0.0);
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
    let mut world = HittableList::new();
    let perlin_texture = Rc::new(Lambertian {
        texture: Rc::new(NoiseTexture {
            noise: PerlinNoise::new(),
            scale: 4.0,
        }),
    });
    let sphere = Rc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_texture.clone(),
    ));
    let ground = Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_texture,
    ));
    world.add(ground);
    world.add(sphere);
    let light_material = Rc::new(Emissive {
        emit_color: Vec3::new(4.0, 4.0, 4.0), // 超乎寻常的亮度
    });
    let light_rectangle = Rc::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        light_material.clone(),
    ));
    let light_sphere = Rc::new(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 2.0, light_material));
    world.add(light_rectangle);
    world.add(light_sphere);
    camera.render(&world);
}
#[allow(dead_code)]
fn quads() {
    let aspect_ration = 1.0;
    let width = 400;
    let samples_per_pixel = 100;
    let camera_max_depth = 50;
    let field_of_view = 80.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let look_from = Vec3::new(0.0, 0.0, 9.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let background = Vec3::new(0.7, 0.8, 1.0);
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
    let mut world = HittableList::new();
    let left_red = Rc::new(Lambertian {
        texture: Rc::new(SolidColor::new(Vec3::new(1.0, 0.2, 0.2))),
    });
    let back_green = Rc::new(Lambertian {
        texture: Rc::new(SolidColor::new(Vec3::new(0.2, 1.0, 0.2))),
    });
    let right_blue = Rc::new(Lambertian {
        texture: Rc::new(SolidColor::new(Vec3::new(0.2, 0.2, 1.0))),
    });
    let upper_orange = Rc::new(Lambertian {
        texture: Rc::new(SolidColor::new(Vec3::new(1.0, 0.5, 0.0))),
    });
    let lower_teal = Rc::new(Lambertian {
        texture: Rc::new(SolidColor::new(Vec3::new(0.2, 0.8, 0.8))),
    });

    let quad0 = Rc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    ));
    let quad1 = Rc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    ));
    let quad2 = Rc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    ));
    let quad3 = Rc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));
    let quad4 = Rc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));
    world.add(quad0);
    world.add(quad1);
    world.add(quad2);
    world.add(quad3);
    world.add(quad4);
    camera.render(&world);
}
#[allow(dead_code)]
fn bouncing_spheres() {
    // 创建相机
    let aspect_ration = 16.0 / 9.0;
    let width = 400;
    let samples_per_pixel = 100;
    let camera_max_depth = 50;
    let field_of_view = 20.0;
    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let background = Vec3::new(0.7, 0.8, 1.0);
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
    // 定义材质
    let texture_odd = Rc::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1)));
    let texture_even = Rc::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9)));
    let material_ground = Rc::new(Lambertian {
        texture: Rc::new(CheckeredTexture::new(texture_odd, texture_even, 0.32)),
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
                    let material = Rc::new(Lambertian {
                        texture: Rc::new(SolidColor::new(albedo)),
                    });
                    let route = Vec3::new(0.0, rng.random_range(0.0..0.5), 0.0);
                    let moving_route = Ray::new(center, route, 0.0);
                    let moving_sphere = Rc::new(MovingSphere::new(moving_route, 0.2, material));
                    world.add(moving_sphere);
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
        texture: Rc::new(SolidColor::new(Vec3::new(0.4, 0.2, 0.1))),
    });
    let sphere2 = Rc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));
    world.add(sphere2);
    let material3 = Rc::new(Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    let sphere3 = Rc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));
    world.add(sphere3);
    let mut bvh_world = HittableList::new();
    let cnt = world.objects.len();
    bvh_world.add(Rc::new(BvhNode::new(&mut world.objects, 0, cnt)));
    // 渲染图片
    camera.render(&world);
}
#[allow(dead_code)]
fn checker_spheres() {
    let aspect_ration = 16.0 / 9.0;
    let width = 400;
    let samples_per_pixel = 100;
    let camera_max_depth = 50;
    let field_of_view = 20.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let background = Vec3::new(0.7, 0.8, 1.0);
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
    let mut world = HittableList::new();
    let texture_odd = Rc::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1)));
    let texture_even = Rc::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9)));
    let checker1 = Rc::new(Lambertian {
        texture: Rc::new(CheckeredTexture::new(texture_odd, texture_even, 0.32)),
    });
    let checker2 = checker1.clone();
    let sphere1 = Rc::new(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, checker1));
    let sphere2 = Rc::new(Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, checker2));
    world.add(sphere1);
    world.add(sphere2);
    camera.render(&world);
}
#[allow(dead_code)]
fn earth() {
    let img_path = Path::new("texture_images/earthmap.jpg");
    let img = ImageReader::open(img_path)
        .expect("文件不存在")
        .decode()
        .expect("图片解码失败");

    let aspect_ration = 16.0 / 9.0;
    let width = img.width();
    let samples_per_pixel = 100;
    let camera_max_depth = 50;
    let field_of_view = 20.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let look_from = Vec3::new(0.0, 0.0, 12.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let background = Vec3::new(0.7, 0.8, 1.00);
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
    let mut world = HittableList::new();
    let earth_texture = Rc::new(ImageTexture::new(img));
    let earth_surface = Rc::new(Lambertian {
        texture: earth_texture,
    });
    let earth = Rc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
    world.add(earth);
    camera.render(&world);
}
#[allow(dead_code)]
fn perlin_sphere() {
    let aspect_ration = 16.0 / 9.0;
    let width = 400;
    let samples_per_pixel = 100;
    let camera_max_depth = 50;
    let field_of_view = 20.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let background = Vec3::new(0.7, 0.8, 1.0);
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
    let mut world = HittableList::new();
    let texture_perlin = Rc::new(Lambertian {
        texture: Rc::new(NoiseTexture {
            noise: PerlinNoise::new(),
            scale: 4.0,
        }),
    });
    let ground = Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        texture_perlin.clone(),
    ));
    let sphere = Rc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, texture_perlin));
    world.add(ground);
    world.add(sphere);
    camera.render(&world);
}
