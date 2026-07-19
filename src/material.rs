use crate::hittable::*;
use crate::ray::Ray;
use crate::texture::*;
use crate::vec3::Vec3;
use rand::{RngExt, rng};
use std::sync::Arc;
pub trait Material: Sync + Send {
    // bool 表示有无表面散射
    fn scatter(
        &self,
        in_ray: &Ray,
        scattered_ray: &mut Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3, // 衰减
    ) -> bool;
    #[allow(unused_variables)]
    fn scattering_pdf(&self, in_ray: &Ray, scattered_ray: &Ray, rec: &HitRecord) -> f64 {
        0.0
    }
    #[allow(unused_variables)]
    fn emitted(&self, u: f64, v: f64, point: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
// 漫反射
pub struct Lambertian {
    pub texture: Arc<dyn Texture>, // 反照率 [0.0,1.0]
}
impl Material for Lambertian {
    fn scatter(
        &self,
        in_ray: &Ray,
        scattered_ray: &mut Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
    ) -> bool {
        // scattered_ray.direction = rec.normal + Vec3::generate_rand_norm(-1.0, 1.0);
        // if scattered_ray.direction.near_zero() {
        //     scattered_ray.direction = rec.normal;
        // }
        let dir = Vec3::generate_rand_norm(-1.0, 1.0);
        if dir * rec.normal < 0.0 {
            scattered_ray.direction = dir * -1.0;
        } else {
            scattered_ray.direction = dir;
        }
        scattered_ray.origin = Vec3::new(rec.hit_point.x, rec.hit_point.y, rec.hit_point.z);
        let texture_value = self.texture.value(rec.u, rec.v, &rec.hit_point);
        attenuation.x = texture_value.x;
        attenuation.y = texture_value.y;
        attenuation.z = texture_value.z;
        scattered_ray.time = in_ray.time;
        true
    }
    #[allow(unused_variables)]
    fn scattering_pdf(&self, in_ray: &Ray, scattered_ray: &Ray, rec: &HitRecord) -> f64 {
        // let cos_theta = rec.normal * scattered_ray.direction.normalize();
        // if cos_theta < 0.0 {
        //     return 0.0;
        // }
        // cos_theta / std::f64::consts::PI
        1.0 / (2.0 * std::f64::consts::PI)
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
        in_ray: &Ray,
        scattered_ray: &mut Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
    ) -> bool {
        scattered_ray.origin = rec.hit_point;
        scattered_ray.direction = Metal::mirror_reflect(in_ray.direction, rec.normal).normalize()
            + Vec3::generate_rand_norm(-1.0, 1.0) * self.fuzz;
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        scattered_ray.time = in_ray.time;
        scattered_ray.direction * rec.normal > 0.0
    }
}

pub struct Dielectrics {
    pub refractive_index: f64, // 折射率
}
impl Dielectrics {
    pub fn refract(refraction_rate: f64, in_direction: Vec3, normal: Vec3) -> Vec3 {
        let unit_in = in_direction.normalize();
        let in_perpendicular = unit_in - normal * (unit_in * normal);
        let out_perpendicular = in_perpendicular / refraction_rate;
        let out_parallel = normal * (1.0 - out_perpendicular.length_squared()).sqrt() * -1.0;
        out_parallel + out_perpendicular
    }
    pub fn reflectance(cosine: f64, refraction: f64) -> f64 {
        let mut r0 = (1.0 - refraction) / (1.0 + refraction);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
impl Material for Dielectrics {
    fn scatter(
        &self,
        in_ray: &Ray,
        scattered_ray: &mut Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
    ) -> bool {
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0;
        scattered_ray.origin = rec.hit_point;

        let unit_in = in_ray.direction.normalize();
        let sin_theta = (unit_in - rec.normal * (unit_in * rec.normal))
            .length_squared()
            .sqrt();
        let cos_theta = (1.0 - sin_theta * sin_theta).sqrt();
        let mut rng = rng();

        #[allow(clippy::collapsible_else_if)]
        if rec.front_face {
            if sin_theta / self.refractive_index > 1.0
                || Dielectrics::reflectance(cos_theta, 1.0 / self.refractive_index)
                    > rng.random_range(0.0..1.0)
            {
                scattered_ray.direction = Metal::mirror_reflect(unit_in, rec.normal);
            } else {
                scattered_ray.direction =
                    Dielectrics::refract(self.refractive_index / 1.0, in_ray.direction, rec.normal);
            }
        } else {
            if sin_theta * self.refractive_index > 1.0
                || Dielectrics::reflectance(cos_theta, self.refractive_index)
                    > rng.random_range(0.0..1.0)
            {
                scattered_ray.direction = Metal::mirror_reflect(unit_in, rec.normal);
            } else {
                scattered_ray.direction =
                    Dielectrics::refract(1.0 / self.refractive_index, in_ray.direction, rec.normal);
            }
        }
        scattered_ray.time = in_ray.time;
        true
    }
}
pub struct Emissive {
    pub emit_color: Vec3,
}
impl Material for Emissive {
    #[allow(unused_variables)]
    fn scatter(
        &self,
        in_ray: &Ray,
        scattered_ray: &mut Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
    ) -> bool {
        false
    }
    #[allow(unused_variables)]
    fn emitted(&self, u: f64, v: f64, point: Vec3) -> Vec3 {
        self.emit_color
    }
}
pub struct Isotropic {
    pub texture: Arc<dyn Texture>,
}
impl Isotropic {
    #[allow(dead_code)]
    pub fn new(texture: Arc<dyn Texture>) -> Isotropic {
        Isotropic { texture }
    }
}
impl Material for Isotropic {
    fn scatter(
        &self,
        in_ray: &Ray,
        scattered_ray: &mut Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
    ) -> bool {
        scattered_ray.origin = rec.hit_point;
        scattered_ray.direction = Vec3::generate_rand_norm(-1.0, 1.0);
        scattered_ray.time = in_ray.time;
        let texture_value = self.texture.value(rec.u, rec.v, &rec.hit_point);
        attenuation.x = texture_value.x;
        attenuation.y = texture_value.y;
        attenuation.z = texture_value.z;
        true
    }
}
