use crate::vec3::*;
use crate::hittable::*;
use crate::ray::*;
use enum_dispatch::enum_dispatch;
use rand::random;

pub struct Lambertian {
    pub albedo: Colour
}

pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f64
}

pub struct Dielectric {
    pub mu: f64
}

pub struct TestMaterial {
    pub albedo: Colour
}

#[enum_dispatch]
pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)>;
}

#[enum_dispatch(Scatter)]
pub enum Material { // only albedo in each material? then just make a generic?
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    TestMaterial(TestMaterial)
}

impl Scatter for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let mut scatter_dir = rec.normal + randvec_in_unit_sphere();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        Some((ray(rec.p, scatter_dir), self.albedo))
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let mut refl = reflect(&r_in.dir(), &rec.normal);
        refl = unit_vector(refl) + (self.fuzz * randvec_in_unit_sphere()); 

        let scatrd_ray = ray(rec.p, refl);
        if dot(&scatrd_ray.dir(), &rec.normal) > 0.0 {
            return Some((scatrd_ray, self.albedo));
        } else {
            return None;
        }
    }
}

impl Scatter for Dielectric {
    fn scatter(&self,r_in: &Ray,rec: &HitRecord) -> Option<(Ray,Colour)> {
        let ri = if rec.front_face { 1.0 / self.mu } else { self.mu };

        let uni_dir = unit_vector(r_in.dir());
        let dotval = dot(&uni_dir, &rec.normal);

        let cos_theta = if dotval < 1.0 { dotval } else { 1.0 };
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let mut ray_dir = refract(&uni_dir, &rec.normal, ri);

        if ri * sin_theta > 1.0 || reflectance(cos_theta, self.mu) > random() {
            ray_dir = reflect(&uni_dir, &rec.normal);
        }

        Some((ray(rec.p, ray_dir), colour(1.0, 1.0, 1.0)))
    }
}

impl Scatter for TestMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray,Colour)> {
        let scatter_dir = rec.normal;
        if rec.front_face {
            return Some((ray(rec.p, scatter_dir), colour(1,1,1)));
        } else {
            return Some((ray(rec.p, scatter_dir), colour(0,0,0)));
        }
    }
}

fn reflectance(cosine: f64, mu: f64) -> f64 {
    // schlick's approximation
    let mut r0 = (1.0 - mu) / (1.0 + mu);
    r0 = r0*r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}