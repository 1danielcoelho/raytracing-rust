use rand::Rng;
use std::f64;
use std::fs;
use std::rc::Rc;

extern crate open;
extern crate raytracer;
extern crate time;

use time::PreciseTime;

use raytracer::camera::Camera;
use raytracer::hitable::{Hitable, HitableList};
use raytracer::material::{Dielectric, Lambertian, Metal};
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

fn random_scene<'a>() -> HitableList<'a> {
    let mut list: Vec<Box<dyn Hitable>> = Vec::new();

    // Floor
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(Vec3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ))),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.5 * (1.0 + rng.gen::<f64>()),
                        )),
                    )));
                } else {
                    // glass
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));

    list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));

    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    return HitableList { list };
}

fn main() {
    let start = PreciseTime::now();

    let nx = 200u32;
    let ny = 100u32;
    let ns = 30000u32;

    let mut output = format!("P3\n{} {}\n255\n", nx, ny);

    let world = random_scene();

    let lookfrom = Vec3::new(12.0, 2.0, 2.9);
    let lookat = Vec3::new(0.0, 0.0, 0.0);

    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f64 / ny as f64,
        0.1,
        (lookfrom - lookat).length(),
    );

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

    let end = PreciseTime::now();
    println!("{} seconds", start.to(end));

    fs::write("test.ppm", output).expect("Failed to write");
    open::that("test.ppm").expect("Failed to open file");
}
