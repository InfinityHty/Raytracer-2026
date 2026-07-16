use crate::axis_aligned_bounding_boxes::AxisAlignedBoundingBox;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::*;
use crate::ray::Ray;
use crate::texture::*;
use crate::vec3::Vec3;
use rand::{RngExt, rng};
use std::cmp::Ordering;
use std::f64::consts::PI;
use std::rc::Rc;

#[allow(unused_variables)]
pub struct HitRecord {
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,           // 是否是外表面
    pub material: Rc<dyn Material>, // 持有材质的指针
    pub u: f64,
    pub v: f64, // 这两个参数对应texture map
}
// 一个可撞击的trait（类比抽象类）
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
    fn get_bounding_box(&self) -> &AxisAlignedBoundingBox;
}
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
    bounding_box: AxisAlignedBoundingBox,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
            bounding_box: AxisAlignedBoundingBox::new(
                Interval::new(center.x - radius, center.x + radius),
                Interval::new(center.y - radius, center.y + radius),
                Interval::new(center.z - radius, center.z + radius),
            ),
        }
    }
    pub fn get_sphere_uv(&self, point: &Vec3) -> (f64, f64) {
        let theta = (-point.y / self.radius).acos();
        let phi = (-point.z).atan2(point.x) + std::f64::consts::PI;
        (
            phi / 2.0 / std::f64::consts::PI,
            theta / std::f64::consts::PI,
        )
    }
    // u,v: [0,1] 对应 phi theta
}
impl Hittable for Sphere {
    // t在某个范围内才能成功hit (t_min,t_max)
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let a = ray.direction().length_squared();
        let h = ray.direction() * (self.center - ray.ori());
        let c = (self.center - ray.ori()).length_squared() - self.radius * self.radius;
        if h * h - a * c < 0.0 {
            return false;
        }
        let det = (h * h - a * c).sqrt();
        let mut rt = (h - det) / a;
        if !ray_t.surround(rt) {
            rt = (h + det) / a;
            if !ray_t.surround(rt) {
                return false;
            }
        }
        rec.t = rt;
        rec.hit_point = ray.at(rec.t);
        let outward_normal = (rec.hit_point - self.center) / self.radius;
        if outward_normal * ray.direction < 0.0 {
            rec.normal = outward_normal;
            rec.front_face = true;
        } else {
            rec.normal = outward_normal * (-1.0);
            rec.front_face = false;
        }
        (rec.u, rec.v) = self.get_sphere_uv(&rec.hit_point);
        rec.material = Rc::clone(&self.material);
        true
    }
    fn get_bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}
// 法线方向始终和光线反向
pub struct MovingSphere {
    center: Ray,
    radius: f64,
    material: Rc<dyn Material>,
    bounding_box: AxisAlignedBoundingBox,
}
impl MovingSphere {
    pub fn new(center: Ray, radius: f64, material: Rc<dyn Material>) -> MovingSphere {
        let min_x = center.origin.x.min(center.origin.x + center.direction.x);
        let max_x = center.origin.x.max(center.origin.x + center.direction.x);
        let min_y = center.origin.y.min(center.origin.y + center.direction.y);
        let max_y = center.origin.y.max(center.origin.y + center.direction.y);
        let min_z = center.origin.z.min(center.origin.z + center.direction.z);
        let max_z = center.origin.z.max(center.origin.z + center.direction.z);
        MovingSphere {
            center,
            radius,
            material,
            bounding_box: AxisAlignedBoundingBox::new(
                Interval::new(min_x - radius, max_x + radius),
                Interval::new(min_y - radius, max_y + radius),
                Interval::new(min_z - radius, max_z + radius),
            ),
        }
    }
}
impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.origin + self.center.direction * ray.time;

        let a = ray.direction().length_squared();
        let h = ray.direction() * (current_center - ray.ori());
        let c = (current_center - ray.ori()).length_squared() - self.radius * self.radius;
        if h * h - a * c < 0.0 {
            return false;
        }
        let det = (h * h - a * c).sqrt();
        let mut rt = (h - det) / a;
        if !ray_t.surround(rt) {
            rt = (h + det) / a;
            if !ray_t.surround(rt) {
                return false;
            }
        }
        rec.t = rt;
        rec.hit_point = ray.at(rec.t);
        let outward_normal = (rec.hit_point - current_center) / self.radius;
        if outward_normal * ray.direction() < 0.0 {
            rec.normal = outward_normal;
            rec.front_face = true;
        } else {
            rec.normal = outward_normal * (-1.0);
            rec.front_face = false;
        }
        rec.material = Rc::clone(&self.material);
        true
    }
    fn get_bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}
// Bounding Volume Hierarchies 优化 二分思想
pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bounding_box: AxisAlignedBoundingBox,
}
impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(ray, ray_t) {
            return false;
        }
        let hit_left = self.left.hit(ray, ray_t, rec);
        let hit_right = self.right.hit(ray, ray_t, rec);
        hit_left || hit_right
    }
    fn get_bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}
impl BvhNode {
    pub fn new(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bounding_box = AxisAlignedBoundingBox::new(
            Interval::new(0.0, 0.0),
            Interval::new(0.0, 0.0),
            Interval::new(0.0, 0.0),
        );
        for it in objects.iter().take(end).skip(start) {
            bounding_box = AxisAlignedBoundingBox::merge(&bounding_box, it.get_bounding_box());
        }
        let axis = bounding_box.longest_axis();

        let span = end - start;
        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;
        if span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            // sort
            if axis == 0 {
                objects[start..end].sort_by(|a, b| {
                    if Self::x_compare(a, b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
            } else if axis == 1 {
                objects[start..end].sort_by(|a, b| {
                    if Self::y_compare(a, b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
            } else {
                objects[start..end].sort_by(|a, b| {
                    if Self::z_compare(a, b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
            }
            let mid = start + span / 2;
            left = Rc::new(BvhNode::new(objects, start, mid));
            right = Rc::new(BvhNode::new(objects, mid, end));
        }
        let bounding_box =
            AxisAlignedBoundingBox::merge(left.get_bounding_box(), right.get_bounding_box());
        Self {
            left,
            right,
            bounding_box,
        }
    }
    #[allow(dead_code)]
    fn rand_int(min: i32, max: i32) -> i32 {
        let mut rng = rng();
        rng.random_range(min..max)
    }
    fn x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> bool {
        a.get_bounding_box().interval_x.min < b.get_bounding_box().interval_x.min
    }
    fn y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> bool {
        a.get_bounding_box().interval_y.min < b.get_bounding_box().interval_y.min
    }
    fn z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> bool {
        a.get_bounding_box().interval_z.min < b.get_bounding_box().interval_z.min
    }
}
pub struct Quad {
    // Ax + by + Cz = d
    point: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    d: f64,
    material: Rc<dyn Material>,
    bounding_box: AxisAlignedBoundingBox,
}
impl Quad {
    pub fn new(point: Vec3, u: Vec3, v: Vec3, material: Rc<dyn Material>) -> Self {
        let point1 = point + u;
        let point2 = point + v;
        let point3 = point + u + v;
        let n = Vec3::cross_multiply(u, v);
        let box1 = AxisAlignedBoundingBox::new_from_points(point, point3);
        let box2 = AxisAlignedBoundingBox::new_from_points(point1, point2);
        Self {
            point,
            u,
            v,
            w: n / (n * n),
            normal: n.normalize(),
            d: point * n.normalize(),
            material,
            bounding_box: AxisAlignedBoundingBox::merge(&box1, &box2),
        }
    }
}
impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        if (self.normal * ray.direction).abs() < 1e-20 {
            return false;
        }
        let t = (self.d - self.normal * ray.origin) / (self.normal * ray.direction);
        if !ray_t.contains(t) {
            false
        } else {
            let intersection = ray.at(t);
            let p = intersection - self.point;
            let alpha = self.w * (Vec3::cross_multiply(p, self.v));
            let beta = self.w * (Vec3::cross_multiply(self.u, p));
            if !(0.0..=1.0).contains(&alpha) || !(0.0..=1.0).contains(&beta) {
                return false;
            }
            rec.t = t;
            rec.hit_point = intersection;
            if ray.direction * self.normal > 0.0 {
                rec.normal = self.normal * -1.0;
                rec.front_face = false;
            } else {
                rec.normal = self.normal;
                rec.front_face = true;
            }
            rec.material = self.material.clone();
            true
        }
    }
    fn get_bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}
// 平移
pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bounding_box: AxisAlignedBoundingBox,
}
impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
        let old_box = object.get_bounding_box();
        let point1 = Vec3::new(
            old_box.interval_x.min,
            old_box.interval_y.min,
            old_box.interval_z.min,
        ) + offset;
        let point2 = Vec3::new(
            old_box.interval_x.max,
            old_box.interval_y.max,
            old_box.interval_z.max,
        ) + offset;
        let bounding_box = AxisAlignedBoundingBox::new_from_points(point1, point2);
        Self {
            object,
            offset,
            bounding_box,
        }
    }
}
impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let ray_moved = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        if !self.object.hit(&ray_moved, ray_t, rec) {
            return false;
        }
        rec.hit_point = rec.hit_point + self.offset;
        true
    }
    fn get_bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}
// 绕y轴
pub struct RotateY {
    object: Rc<dyn Hittable>,
    cos_theta: f64,
    sin_theta: f64,
    bounding_box: AxisAlignedBoundingBox,
}
impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> Self {
        let cos_theta = (angle / 180.0 * PI).cos();
        let sin_theta = (angle / 180.0 * PI).sin();

        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);
        let old_box = object.get_bounding_box();

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x =
                        i as f64 * old_box.interval_x.min + (1 - i) as f64 * old_box.interval_x.max;
                    let y =
                        j as f64 * old_box.interval_y.min + (1 - j) as f64 * old_box.interval_y.max;
                    let z =
                        k as f64 * old_box.interval_z.min + (1 - k) as f64 * old_box.interval_z.max;

                    let new_x = x * cos_theta + z * sin_theta;
                    let new_z = x * -sin_theta + z * cos_theta;

                    let new_point = Vec3::new(new_x, y, new_z);
                    min.x = min.x.min(new_point.x);
                    max.x = max.x.max(new_point.x);
                    min.y = min.y.min(new_point.y);
                    max.y = max.y.max(new_point.y);
                    min.z = min.z.min(new_point.z);
                    max.z = max.z.max(new_point.z);
                }
            }
        }
        let bounding_box = AxisAlignedBoundingBox::new_from_points(min, max);
        Self {
            object,
            cos_theta,
            sin_theta,
            bounding_box,
        }
    }
}
impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let rotated_origin = Vec3::new(
            ray.origin.x * self.cos_theta - ray.origin.z * self.sin_theta,
            ray.origin.y,
            ray.origin.x * self.sin_theta + ray.origin.z * self.cos_theta,
        );
        let rotated_direction = Vec3::new(
            ray.direction.x * self.cos_theta - ray.direction.z * self.sin_theta,
            ray.direction.y,
            ray.direction.x * self.sin_theta + ray.direction.z * self.cos_theta,
        );
        let rotated_ray = Ray::new(rotated_origin, rotated_direction, ray.time);

        if !self.object.hit(&rotated_ray, ray_t, rec) {
            return false;
        }
        rec.hit_point = Vec3::new(
            rec.hit_point.x * self.cos_theta + rec.hit_point.z * self.sin_theta,
            rec.hit_point.y,
            rec.hit_point.x * -self.sin_theta + rec.hit_point.z * self.cos_theta,
        );
        rec.normal = Vec3::new(
            rec.normal.x * self.cos_theta + rec.normal.z * self.sin_theta,
            rec.normal.y,
            rec.normal.x * -self.sin_theta + rec.normal.z * self.cos_theta,
        );
        true
    }
    fn get_bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}
pub struct Cube {
    sides: HittableList,
    bounding_box: AxisAlignedBoundingBox,
}
impl Hittable for Cube {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        self.sides.hit(ray, ray_t, rec)
    }
    fn get_bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}
impl Cube {
    pub fn new(point1: Vec3, point2: Vec3, mat: Rc<dyn Material>) -> Self {
        let min = Vec3::new(
            point1.x.min(point2.x),
            point1.y.min(point2.y),
            point1.z.min(point2.z),
        );
        let max = Vec3::new(
            point1.x.max(point2.x),
            point1.y.max(point2.y),
            point1.z.max(point2.z),
        );
        let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y - min.y, 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z - min.z);
        let mut sides = HittableList::new();
        sides.add(Rc::new(Quad::new(min, dx, dy, mat.clone())));
        sides.add(Rc::new(Quad::new(min, dx, dz, mat.clone())));
        sides.add(Rc::new(Quad::new(min, dy, dz, mat.clone())));
        sides.add(Rc::new(Quad::new(max, dx * -1.0, dy * -1.0, mat.clone())));
        sides.add(Rc::new(Quad::new(max, dx * -1.0, dz * -1.0, mat.clone())));
        sides.add(Rc::new(Quad::new(max, dy * -1.0, dz * -1.0, mat.clone())));
        let bounding_box = AxisAlignedBoundingBox::new_from_points(min, max);
        Self {
            sides,
            bounding_box,
        }
    }
}
pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Rc<dyn Material>,
}
impl ConstantMedium {
    pub fn new(
        boundary: Rc<dyn Hittable>,
        density: f64,
        phase_function: Rc<dyn Material>,
    ) -> ConstantMedium {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }
}
impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord {
            hit_point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
            material: Rc::new(Lambertian {
                texture: Rc::new(SolidColor::new(Vec3::new(0.0, 0.0, 0.0))),
            }),
            u: 0.0,
            v: 0.0,
        };
        let mut rec2 = HitRecord {
            hit_point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
            material: Rc::new(Lambertian {
                texture: Rc::new(SolidColor::new(Vec3::new(0.0, 0.0, 0.0))),
            }),
            u: 0.0,
            v: 0.0,
        };
        if !self.boundary.hit(
            ray,
            &Interval::new(-f64::INFINITY, f64::INFINITY),
            &mut rec1,
        ) {
            return false;
        }
        if !self.boundary.hit(
            ray,
            &Interval::new(rec1.t + 0.0001, f64::INFINITY),
            &mut rec2,
        ) {
            return false;
        }
        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }
        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = ray.direction.length_squared().sqrt();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let mut rng = rng();
        let hit_distance = self.neg_inv_density * (rng.random_range(0.00001..1.0) as f64).log2();
        if hit_distance > distance_inside_boundary {
            return false;
        }
        rec.t = rec1.t + hit_distance / ray_length;
        rec.hit_point = ray.at(rec.t);
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.material = self.phase_function.clone();
        rec.front_face = true;
        true
    }
    fn get_bounding_box(&self) -> &AxisAlignedBoundingBox {
        self.boundary.get_bounding_box()
    }
}
