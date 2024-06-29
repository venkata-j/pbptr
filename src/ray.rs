use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Ray {
    ori: Point3,
    dir: Point3
}

impl Ray {
    pub fn new(orig: Point3, dire: Vec3) -> Self {
        Self { ori: orig, dir: dire }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.ori + t*self.dir
    }

    pub fn ori(&self) -> Point3 {
        self.ori
    }

    pub fn dir(&self) -> Point3 {
        self.dir
    }
}

#[inline(always)]
pub fn ray(orig: Point3, dire: Vec3) -> Ray {
    Ray::new(orig, dire)
}