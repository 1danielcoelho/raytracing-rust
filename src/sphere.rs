use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere<'a> {
    pub center: Vec3,
    pub radius: f64,
    pub material: &'a Material,
}

impl<'a> Sphere<'a> {
    pub fn new<T>(center: Vec3, radius: f64, material: &T) -> Sphere
    where
        T: Material + 'a,
    {
        return Sphere {
            center,
            radius,
            material,
        };
    }
}

impl<'a> Hitable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &ray.orig - &self.center;
        let a = Vec3::dot(ray.dir, ray.dir);
        let b = Vec3::dot(oc, ray.dir);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t_1 = (-b - (b * b - a * c).sqrt()) / a;

            if t_1 < t_max && t_1 > t_min {
                let t_1_p = ray.point_at_parameter(t_1);
                return Some(HitRecord {
                    t: t_1,
                    p: t_1_p,
                    normal: (t_1_p - self.center) / self.radius,
                    mat_ptr: self.material,
                });
            }

            let t_2 = (-b + (b * b - a * c).sqrt()) / a;

            if t_2 < t_max && t_2 > t_min {
                let t_2_pt = ray.point_at_parameter(t_2);
                return Some(HitRecord {
                    t: t_2,
                    p: t_2_pt,
                    normal: (t_2_pt - self.center) / self.radius,
                    mat_ptr: self.material,
                });
            }
        }

        return None;
    }
}
