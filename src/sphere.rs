use std::rc::Rc;

use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        return Sphere {
            center,
            radius,
            material,
        };
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &ray.orig - &self.center;
        let a = ray.dir.dot(&ray.dir);
        let b = oc.dot(&ray.dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t_1 = (-b - (b * b - a * c).sqrt()) / a;

            if t_1 < t_max && t_1 > t_min {
                let t_1_p = ray.point_at_parameter(t_1);
                return Some(HitRecord {
                    t: t_1,
                    p: t_1_p,
                    normal: (t_1_p - self.center) / self.radius,
                    mat_ptr: self.material.as_ref(),
                });
            }

            let t_2 = (-b + (b * b - a * c).sqrt()) / a;

            if t_2 < t_max && t_2 > t_min {
                let t_2_pt = ray.point_at_parameter(t_2);
                return Some(HitRecord {
                    t: t_2,
                    p: t_2_pt,
                    normal: (t_2_pt - self.center) / self.radius,
                    mat_ptr: self.material.as_ref(),
                });
            }
        }

        return None;
    }
}
