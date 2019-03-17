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

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);

        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - n * v.dot(n) * 2.0;
}

pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.normalized();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        return Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt());
    } else {
        return None;
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    return r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0));
}
