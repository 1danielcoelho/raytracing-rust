use rand::Rng;

use crate::vec3::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        // Random point within [-1, 1] cube
        let p = Vec3::new(
            2.0 * rng.gen::<f64>() - 1.0,
            2.0 * rng.gen::<f64>() - 1.0,
            2.0 * rng.gen::<f64>() - 1.0,
        );

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - n * Vec3::dot(*v, *n) * 2.0;
}
