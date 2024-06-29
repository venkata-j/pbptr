use std::f64::consts::PI;
use std::f64::INFINITY;

#[inline(always)]
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub struct Interval {
    min: f64,
    max: f64
}

impl Interval {
    pub fn all() -> Self {
        Self { min: -INFINITY, max: INFINITY }
    }

    pub fn new(min: f64, max: f64) -> Self {
        Self { min: min, max: max }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn has(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn interior(&self, x: f64) -> bool {
        x > self.min && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { return self.min; }
        if x > self.max { return self.max; }
        return x;
    }

    // pub const EMPTY: Self = Self { max: -INFINITY, min: INFINITY };
    // pub const UNIVE: Self = Self { min: -INFINITY, max: INFINITY };
} // replace with Range<f64>?

pub fn interval(mi: f64, ma: f64) -> Interval {
    Interval { min: mi, max: ma }
}