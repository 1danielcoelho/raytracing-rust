use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    // vfov is top to bottom in degrees
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).normalized();
        let u = vup.cross(&w).normalized();
        let v = w.cross(&u);

        // println!("w: {:?}\nu: {:?}\nv: {:?}", w, u, v);
        // println!(
        //     "horizontal: {:?}\nvertical: {:?}\nlower_left: {:?}",
        //     u * 2.0 * half_width,
        //     v * 2.0 * half_height,
        //     lookfrom - u * half_width - v * half_height - w
        // );

        return Camera {
            lower_left_corner: lookfrom - u * half_width - v * half_height - w,
            horizontal: u * 2.0 * half_width,
            vertical: v * 2.0 * half_height,
            origin: lookfrom,
        };
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin,
        );
    }
}
