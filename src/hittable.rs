use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;
pub struct HitRecord {
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool, // 是否是外表面
}
// 一个可撞击的trait（类比抽象类）
pub trait Hittable {
    fn hit(&self,ray:&Ray,ray_t: &Interval,rec:&mut HitRecord) -> bool;
}
pub struct Sphere{
    center: Vec3,
    radius: f64,
}
impl Sphere{
    pub fn new(center: Vec3, radius: f64) -> Sphere{
        Sphere{center, radius}
    }
}
impl Hittable for Sphere{
    // t在某个范围内才能成功hit (t_min,t_max)
    fn hit(&self,ray: &Ray,ray_t: &Interval,rec:&mut HitRecord) -> bool {
        let a = ray.direction().length_squared();
        let h = ray.direction() * (self.center - ray.ori());
        let c = (self.center - ray.ori()).length_squared() - self.radius * self.radius;
        if h * h - a * c < 0.0{
            return false;
        }
        let det = (h * h - a * c).sqrt();
        let mut rt = (h - det) / a;
        if ray_t.surround(rt) == false{
            rt = (h + det) / a;
            if ray_t.surround(rt) == false{
                return false;
            }
        }
        rec.t = rt;
        rec.hit_point = ray.at(rec.t);
        let outward_normal = (rec.hit_point - self.center) / self.radius;
        if outward_normal * ray.direction() < 0.0{
            rec.normal = outward_normal;
            rec.front_face = true;
        }
        else{
            rec.normal = outward_normal * (-1.0);
            rec.front_face = false;
        }
        true
    }
}
