use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use std::rc::Rc;
use std::vec::Vec;
// 场景里所有的objects
// t_min,t_max用来找光线hit的最近的object
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
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
