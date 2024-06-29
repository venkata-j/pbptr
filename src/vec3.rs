use std::{fs::File, ops, os::windows::thread};
use std::io::Write;
use rand::{thread_rng, Rng};
use std::iter::Sum;
use crate::Interval;

#[derive(Copy, Clone)]
pub struct Vec3 {
    e : [f64; 3]
}

impl Vec3 {
    pub fn new(e1: impl Into<f64>, e2: impl Into<f64>, e3: impl Into<f64>) -> Self {
        Self { e: [e1.into(), e2.into(), e3.into()] }
    }

    // pub fn new() -> Self {
    //     Self { e: [0, 0, 0] }
    // }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn norm_sq(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        const S: f64 = 1.0e-8;

        (self.e[0] < S) && (self.e[1] < S) && (self.e[2] < S)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        Self { e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] }
    }
}

impl ops::AddAssign for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self { e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]] };
    }
}

impl ops::SubAssign for Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self { e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]] };
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Self { e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]] }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Self) -> Self::Output {
        Self { e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]] }
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: f64) -> Self::Output {
        Self { e: [self.e[0] + other, self.e[1] + other, self.e[2] + other] }
    }
}

impl ops::Add<Vec3> for f64 {
    type Output = Vec3;
    #[inline(always)]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self + other.x(), self + other.y(), self + other.z())
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: f64) -> Self::Output {
        Self { e: [self.e[0] * other, self.e[1] * other, self.e[2] * other] }
    }
}
// implement mul by i32

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x(), self * other.y(), self * other.z())
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: f64) -> Self::Output {
        Self { e: [self.e[0] / other, self.e[1] / other, self.e[2] / other] }
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self { e: [0.0, 0.0, 0.0] }, |a, b| a + b)
    }
}

#[inline(always)]
pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
    a.e[0] * b.e[0] + a.e[1] * b.e[1] + a.e[2] * b.e[2]
}

#[inline(always)]
pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3::new(
        a.e[1] * b.e[2] - a.e[2] * b.e[1],
        a.e[2] * b.e[0] - a.e[0] * b.e[2],
        a.e[0] * b.e[1] - a.e[1] * b.e[0]
    )
}

#[inline(always)]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.norm()
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] + rhs, self.e[1] + rhs, self.e[2] + rhs]
        };
    }
}

impl ops::SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] - rhs, self.e[1] - rhs, self.e[2] - rhs]
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs]
        };
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs]
        };
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

pub type Point3 = Vec3;
pub type Colour = Vec3;

#[inline(always)]
fn linear_to_gamma(lincomp: f64) -> f64 { // gamma correction
    if lincomp > 0.0 { return lincomp.sqrt() }
    0.0
}

pub fn write_colour(pixel_colour: Colour, write_out: &mut std::io::BufWriter<File>) {
    let r = linear_to_gamma(pixel_colour.x());
    let g = linear_to_gamma(pixel_colour.y());
    let b = linear_to_gamma(pixel_colour.z());

    let INTENSITY: Interval = Interval::new(0.000, 0.999); // make a static?
    // let rbyte = 256 * (INTENSITY.clamp(r)) as i32;
    // let gbyte = 256 * (INTENSITY.clamp(g)) as i32;
    // let bbyte = 256 * (INTENSITY.clamp(b)) as i32;
    // ^ this doesnt work for some reason, below does though
    let rbyte = (255.999 * INTENSITY.clamp(r)) as i32;
    let gbyte = (255.999 * INTENSITY.clamp(g)) as i32;
    let bbyte = (255.999 * INTENSITY.clamp(b)) as i32;

    writeln!(write_out, "{} {} {}", rbyte, gbyte, bbyte).unwrap(); // use BufWrite
    // println!("{} {} {}", rbyte, gbyte, bbyte);
}

#[inline(always)]
pub fn vec3(a: impl Into<f64>, b: impl Into<f64>, c: impl Into<f64>) -> Vec3 {
    Vec3::new(a, b, c)
}

#[inline(always)]
pub fn point3(a: impl Into<f64>, b: impl Into<f64>, c: impl Into<f64>) -> Point3 {
    Vec3::new(a, b, c)
}

#[inline(always)]
pub fn colour(a: impl Into<f64>, b: impl Into<f64>, c: impl Into<f64>) -> Colour {
    Vec3::new(a, b, c)
}

pub fn randvec() -> Vec3 {
    let mut rng = thread_rng();
    let a: f64 = rng.gen();
    let b: f64 = rng.gen();
    let c: f64 = rng.gen();
    vec3(a, b, c)
}

pub fn randvecr(min: f64, max: f64) -> Vec3 {
    let mut rng = thread_rng();
    let a: f64 = rng.gen_range(min..max);
    let b: f64 = rng.gen_range(min..max);
    let c: f64 = rng.gen_range(min..max);
    vec3(a, b, c)
}

#[inline(always)]
pub fn randvec_in_unit_sphere() -> Vec3 {
    loop {
        let p = randvecr(-1.0, 1.0);
        if p.norm_sq() < 1.0 { return unit_vector(p); }
    }
}

#[inline(always)]
pub fn randvec_on_hemisphere(normal: &Vec3) -> Vec3 {
    // returns a random unit vector on unit sphere, such that it is
    // in the hemisphere facing the source of rays
    let rvec = randvec_in_unit_sphere();
    return if dot(&rvec, normal) > 0.0 { rvec } else { -rvec };
}

#[inline(always)]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(v, n) * *n
}

#[inline(always)]
pub fn refract(uv: &Vec3, n: &Vec3, frac_eta_i_eta_t: f64) -> Vec3 {
    let dotval = dot(&(-(*uv)), n);
    let cos_theta = if dotval < 1.0 { dotval } else { 1.0 };
    let r_out_perp = frac_eta_i_eta_t * (*uv + cos_theta * *n);
    let r_out_prll = -(1.0 - r_out_perp.norm_sq()).abs().sqrt() * *n;

    r_out_perp + r_out_prll
}

#[inline(always)]
pub fn randvec_in_unit_disc() -> Vec3 {
    loop {
        let mut rng = thread_rng();
        let p = vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0);
        if p.norm_sq() < 1.0 { return p; }
    }
}