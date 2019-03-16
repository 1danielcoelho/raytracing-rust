// This is a crate root for the executable

use rand::Rng;
use std::f64;
use std::fs;

// This tells it to pull the external library crate raytracer
extern crate raytracer;

// This tells it to use these types defined in the crate
use raytracer::camera::Camera;
use raytracer::hitable::{Hitable, HitableList};
use raytracer::material::{Lambertian, Metal};
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::vec3::Vec3;

fn color<T: Hitable>(ray: &Ray, world: &T, depth: i32) -> Vec3 {
    match world.hit(ray, 0.001, f64::MAX) {
        Some(rec) => {
            if depth >= 50 {
                return Vec3::new(0.0, 0.0, 0.0);
            } else if let Some(scat) = rec.mat_ptr.scatter(ray, &rec) {
                return color(&scat.out_ray, world, depth + 1) * scat.attenuation;
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }
        None => {
            let unit_dir = ray.dir.normalized();
            let t: f64 = 0.5 * (unit_dir.y + 1.0);
            return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
        }
    }
}

fn main() {
    let nx = 200u32;
    let ny = 100u32;
    let ns = 100u32;

    let mut output = format!("P3\n{} {}\n255\n", nx, ny);

    let l1 = Lambertian::new(Vec3::new(0.8, 0.3, 0.3));
    let l2 = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let m1 = Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0);
    let m2 = Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3);

    let list: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &l1)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &l2)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &m1)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &m2)),
    ];

    let world = HitableList { list };

    let cam = Camera::new();

    let mut rng = rand::thread_rng();

    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;

                let r = cam.get_ray(u, v);
                //let p = r.point_at_parameter(2.0);
                col += color(&r, &world, 0);
            }

            col /= ns as f64;
            col.x = col.x.sqrt();
            col.y = col.y.sqrt();
            col.z = col.z.sqrt();

            let ir = (255.99 * col.r()) as u8;
            let ig = (255.99 * col.g()) as u8;
            let ib = (255.99 * col.b()) as u8;

            output = output + &format!("{} {} {}\n", ir, ig, ib);
        }
    }

    fs::write("test.ppm", output).expect("Failed to write");
}
