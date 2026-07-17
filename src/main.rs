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
    let mut world = HittableList::new();
    // 读取命令行数据 支持多个模型
    let obj_files: Vec<String> = std::env::args().skip(1).collect();

    for obj_file in obj_files {
        // 加载.obj文件
        let (models, _materials) = tobj::load_obj(&obj_file, &tobj::LoadOptions::default())
            .expect("Failed to OBJ load file");

        for m in models.iter() {
            let mesh = &m.mesh;
            let mut next_face = 0;
            for face in 0..mesh.face_arities.len() {
                let end = next_face + mesh.face_arities[face] as usize;

                let face_indices = &mesh.indices[next_face..end];

                if face_indices.len() >= 3 {
                    let point0 = Vec3::new(
                        mesh.positions[(face_indices[0] * 3) as usize] as f64,
                        mesh.positions[(face_indices[0] * 3 + 1) as usize] as f64,
                        mesh.positions[(face_indices[0] * 3 + 2) as usize] as f64,
                    );
                    let point1 = Vec3::new(
                        mesh.positions[(face_indices[1] * 3) as usize] as f64,
                        mesh.positions[(face_indices[1] * 3 + 1) as usize] as f64,
                        mesh.positions[(face_indices[1] * 3 + 2) as usize] as f64,
                    );
                    let point2 = Vec3::new(
                        mesh.positions[(face_indices[2] * 3) as usize] as f64,
                        mesh.positions[(face_indices[2] * 3 + 1) as usize] as f64,
                        mesh.positions[(face_indices[2] * 3 + 2) as usize] as f64,
                    );
                    world.add(Arc::new(Triangle::new(
                        point0,
                        point1 - point0,
                        point2 - point0,
                        Arc::new(Lambertian {
                            texture: Arc::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9))),
                        }),
                    )));

                    if face_indices.len() == 4 {
                        let point3 = Vec3::new(
                            mesh.positions[(face_indices[3] * 3) as usize] as f64,
                            mesh.positions[(face_indices[3] * 3 + 1) as usize] as f64,
                            mesh.positions[(face_indices[3] * 3 + 2) as usize] as f64,
                        );
                        world.add(Arc::new(Triangle::new(
                            point3,
                            point1 - point3,
                            point2 - point3,
                            Arc::new(Lambertian {
                                texture: Arc::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9))),
                            }),
                        )));
                    }
                }
                next_face = end;
            }
        }
    }

    let aspect_ration = 1.0;
    let width = 400;
    let samples_per_pixel = 100;
    let camera_max_depth = 40;
    let field_of_view = 40.0;
    let defocus_angle = 0.0;
    let focus_dist = 5.0;
    let look_from = Vec3::new(5.0, 0.0, 5.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let background = Vec3::new(1.0, 1.0, 1.0);
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
#[allow(dead_code)]
fn final_scene2() {
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
