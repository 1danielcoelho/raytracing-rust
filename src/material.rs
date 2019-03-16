use rand::Rng;

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

        if scattered.dir.dot(&rec.normal) > 0.0 {
            return Some(ScatteredRay {
                out_ray: scattered,
                attenuation: self.albedo,
            });
        }

        return None;
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Dielectric {
        return Dielectric { ref_idx: ri };
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let outward_normal: Vec3;
        let reflected = utils::reflect(&r_in.dir, &rec.normal);
        let ni_over_nt: f64;
        let reflect_prob: f64;
        let cosine: f64;

        if r_in.dir.dot(&rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.dir.dot(&rec.normal) / r_in.dir.length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.dir.dot(&rec.normal) / r_in.dir.length();
        }

        let mut refracted = Vec3::new(1.0, 0.0, 0.0);
        let scattered: Ray;
        match utils::refract(&r_in.dir, &outward_normal, ni_over_nt) {
            Some(refr) => {
                reflect_prob = utils::schlick(cosine, self.ref_idx);
                refracted = refr;
            }
            None => {
                //scattered = Ray::new(rec.p, reflected);
                reflect_prob = 1.0;
            }
        };

        if rand::thread_rng().gen::<f64>() < reflect_prob {
            scattered = Ray::new(rec.p, reflected);
        } else {
            scattered = Ray::new(rec.p, refracted);
        }

        return Some(ScatteredRay {
            out_ray: scattered,
            attenuation: Vec3::new(1.0, 1.0, 1.0),
        });
    }
}
