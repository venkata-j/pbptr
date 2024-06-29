use crate::vec3::*;
use crate::ray::*;
use crate::util::*;
use crate::material::*;
use std::f64::EPSILON;
use std::rc::Rc;
use std::sync::Arc;
use std::vec::Vec;
use enum_dispatch::enum_dispatch;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<Material>
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<Material>
}

pub struct Triangle {
    v1: Point3,
    v2: Point3,
    v3: Point3,
    material: Arc<Material>
}

pub struct HittableList {
    objects: Vec<Arc<Hittable>>
}

#[enum_dispatch]
pub trait Hit {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

#[enum_dispatch(Hit)]
pub enum Hittable {
    Sphere(Sphere),
    HittableList(HittableList),
    Triangle(Triangle)
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.ori();
        let a = r.dir().norm_sq();
        let h = dot(&r.dir(), &oc);
        let c = oc.norm_sq() - self.radius*self.radius;

        let d = h * h - a * c;
        if d >= 0.0 {
            let sqrtd = d.sqrt();
            let mut root = (h - sqrtd) / a;

            if !ray_t.interior(root) {
                root = (h + sqrtd) / a;
                if !ray_t.interior(root) {
                    return None;
                }
            }
            let outward_normal = (r.at(root) - self.center) / self.radius;
            let mut rec = HitRecord::new(r, outward_normal, r.at(root), root, self.material.clone()); // performance?
            rec.set_outward_normal();

            return Some(rec);
        } else {
            return None;
        }
    }
}

impl Hit for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // let mut hit_anything = false;
        let mut closest_so_far = ray_t.max();

        let mut temprec: Option<HitRecord> = None;

        for obj in &self.objects {
            if let Some(obje) = obj.hit(r, interval(ray_t.min(), closest_so_far)) {
                closest_so_far = obje.t;
                temprec = Some(obje);
            }
        }

        temprec
    }
}
// TODO: fix rendering for Metal material
// this code still doesn't work properly
// maybe normal vector direction isn't right? <----- yes, it isn't, why?
impl Hit for Triangle {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // moeller-trumbore
        let e1 = self.v2 - self.v1; // edge_1
        let e2 = self.v3 - self.v1;
        let r_x_e2 = cross(&r.dir(), &e2);
        let dot_prod = dot(&e1, &r_x_e2); // ray.dir() dot normal vec
        
        if dot_prod > -EPSILON && dot_prod < EPSILON {
            return None; // ray is parallel, no intersection
        }

        let s = r.ori() - self.v1;
        let inv_dot = 1.0 / dot_prod;
        let u = inv_dot * dot(&s, &r_x_e2);

        if u > 1.0 || u < 0.0 { return None; } // outside

        let s_x_e1 = cross(&s, &e1);
        let v = inv_dot * dot(&r.dir(), &s_x_e1);

        if v < 0.0 || (u + v) > 1.0 { return None; } // outside

        let t = inv_dot * dot(&e2, &s_x_e1);

        // if ray_t.interior(t) { // > EPSILON??
        if t > (ray_t.min() + EPSILON) && t < (ray_t.max() - EPSILON) {
            let sign = if dot_prod < 0.0 { -1.0 } else { 1.0 };
            // assert_eq!(dot_prod < -EPSILON, dot(&r.dir(), &cross(&e1, &e2)) < -EPSILON);
            // ^ why doesn't this work?
            let mut rec = HitRecord::new( 
                r,
                sign * unit_vector(cross(&e1, &e2)), // <- is this correct?
                r.at(t),
                t,
                self.material.clone()
            );
            rec.set_outward_normal(); // this has no effect, why?
            return Some(rec);
            // change code to include correct normal?
        } else { return None; }
    }
}

impl HitRecord {
    pub fn new(r: &Ray, normal: Vec3, po: Point3, tt: f64, mat: Arc<Material>) -> Self {
        Self {
            p: po,
            t: tt,
            front_face: dot(&r.dir(), &normal) < 0.0,
            normal: normal,
            material: mat
        }
        // performance issue?
    }

    pub fn set_outward_normal(&mut self) {
        self.normal =  if self.front_face { self.normal } else { -self.normal };
    }
}

impl Sphere {
    pub fn new(ctr: Point3, rad: f64, material: Arc<Material>) -> Self {
        Self { center: ctr, radius: rad, material: material }
    }
}

impl Triangle {
    pub fn new(a: Point3, b: Point3, c: Point3, mat: Arc<Material>) -> Self {
        Self { v1: a, v2: b, v3: c, material: mat }
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Arc<Hittable>) {
        self.objects.push(obj);
    }

    pub fn add_front(&mut self, obj: Arc<Hittable>) {
        self.objects.splice(0..0, [obj]);
    }
}
