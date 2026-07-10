use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::Vec3;
pub trait Material {
    // bool 表示有无表面散射
    fn scatter(
        &self,
        _in_ray: &Ray,
        scattered_ray: &mut Ray,
        rec: &HitRecord,
        reflect_rate: &mut Vec3,
    ) -> bool;
}
// 漫反射
pub struct Lambertian {
    pub albedo: Vec3, // 反射光/入射光 [0.0,1.0]
}
impl Material for Lambertian {
    fn scatter(
        &self,
        _in_ray: &Ray,
        scattered_ray: &mut Ray,
        rec: &HitRecord,
        reflect_rate: &mut Vec3,
    ) -> bool {
        scattered_ray.direction = rec.normal + Vec3::generate_rand_norm(-1.0, 1.0);
        if scattered_ray.direction.near_zero() {
            scattered_ray.direction = rec.normal;
        }
        scattered_ray.origin = Vec3::new(rec.hit_point.x, rec.hit_point.y, rec.hit_point.z);
        reflect_rate.x = self.albedo.x;
        reflect_rate.y = self.albedo.y;
        reflect_rate.z = self.albedo.z;
        true
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64, // [0,1]
}
impl Metal {
    pub fn mirror_reflect(in_direction: Vec3, normal: Vec3) -> Vec3 {
        in_direction - normal * (in_direction * normal) * 2.0
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        _in_ray: &Ray,
        scattered_ray: &mut Ray,
        rec: &HitRecord,
        reflect_rate: &mut Vec3,
    ) -> bool {
        scattered_ray.origin = rec.hit_point;
        scattered_ray.direction = Metal::mirror_reflect(_in_ray.direction, rec.normal).normalize()
            + Vec3::generate_rand_norm(-1.0, 1.0) * self.fuzz;
        reflect_rate.x = self.albedo.x;
        reflect_rate.y = self.albedo.y;
        reflect_rate.z = self.albedo.z;
        scattered_ray.direction * rec.normal > 0.0
    }
}
