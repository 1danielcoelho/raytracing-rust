use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        return Ray { orig, dir };
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        return self.orig + self.dir * t;
    }
}
