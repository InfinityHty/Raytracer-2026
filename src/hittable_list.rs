use crate::axis_aligned_bounding_boxes::AxisAlignedBoundingBox;
use crate::hittable::Hittable;
use crate::hittable::*;
use crate::interval::*;
use crate::material::*;
use crate::ray::*;
use crate::vec3::Vec3;
use std::rc::Rc;
use std::vec::Vec;
// 场景里所有的objects
// t_min,t_max用来找光线hit的最近的object
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    pub bounding_box: AxisAlignedBoundingBox,
}
impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec![],
            bounding_box: AxisAlignedBoundingBox::new(
                Interval::new(0.0, 0.0),
                Interval::new(0.0, 0.0),
                Interval::new(0.0, 0.0),
            ),
        }
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bounding_box =
            AxisAlignedBoundingBox::merge(&self.bounding_box, object.get_bounding_box());
        self.objects.push(object);
    }
    pub fn add_box(&mut self, point1: Vec3, point2: Vec3, mat: Rc<dyn Material>) {
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
        self.add(Rc::new(Quad::new(min, dx, dy, mat.clone())));
        self.add(Rc::new(Quad::new(min, dx, dz, mat.clone())));
        self.add(Rc::new(Quad::new(min, dy, dz, mat.clone())));
        self.add(Rc::new(Quad::new(max, dx * -1.0, dy * -1.0, mat.clone())));
        self.add(Rc::new(Quad::new(max, dx * -1.0, dz * -1.0, mat.clone())));
        self.add(Rc::new(Quad::new(max, dy * -1.0, dz * -1.0, mat.clone())));
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        for object in &self.objects {
            let tmp_interval = Interval::new(ray_t.min, closest_so_far);
            if object.hit(ray, &tmp_interval, rec) {
                closest_so_far = rec.t;
                hit_anything = true;
            }
        }
        hit_anything
    }
}
