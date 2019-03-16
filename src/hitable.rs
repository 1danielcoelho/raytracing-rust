use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: &'a Material,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitableList<'a> {
    pub list: Vec<Box<dyn Hitable + 'a>>,
}

impl<'a> Hitable for HitableList<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for item in self.list.iter() {
            if let Some(record) = item.hit(r, t_min, closest_so_far) {
                closest_so_far = record.t;
                temp_rec = Some(record);
            }
        }

        return temp_rec;
    }
}
