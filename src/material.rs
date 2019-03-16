use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::utils;
use crate::vec3::Vec3;

pub struct ScatteredRay {
    pub out_ray: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        return Lambertian { albedo };
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let target = rec.p + rec.normal + utils::random_in_unit_sphere();

        return Some(ScatteredRay {
            out_ray: Ray::new(rec.p, target - rec.p),
            attenuation: self.albedo,
        });
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        return Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        };
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let reflected = utils::reflect(&r_in.dir.normalized(), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + utils::random_in_unit_sphere() * self.fuzz,
        );

        if Vec3::dot(scattered.dir, rec.normal) > 0.0 {
            return Some(ScatteredRay {
                out_ray: scattered,
                attenuation: self.albedo,
            });
        }

        return None;
    }
}
