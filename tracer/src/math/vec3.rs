use std::ops;
use rand::random;
use crate::math::utils::random_f64_in_range;

use super::consts::NEAR_ZERO_THRESHOLD;

#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    e: [f64; 3]
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn origin() -> Vec3 { Vec3{e: [0.0, 0.0, 0.0]} }
    pub fn zero() -> Vec3 { Vec3{e: [0.0, 0.0, 0.0]} }
    
    pub fn new<T>(x: T, y: T, z: T) -> Vec3
    where T: Into<f64>, {
        Vec3 { e: [x.into(), y.into(), z.into()] }
    }

    pub const fn new_const(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3{e: [e1, e2, e3]} 
    }

    pub fn random() -> Vec3 { 
        Self::new(
            random::<f64>(),
            random::<f64>(),
            random::<f64>()
        )
    }
    pub fn random_range(min: f64, max: f64) -> Vec3 { 
        Self::new(
            random_f64_in_range(min, max),
            random_f64_in_range(min, max),
            random_f64_in_range(min, max)
        )
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn near_zero(&self) -> bool {
        let thresh = NEAR_ZERO_THRESHOLD;
        self.e[0].abs() < thresh && self.e[1].abs() < thresh && self.e[2].abs() < thresh
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn dot(&self, v: Vec3) -> f64 {
        self.x() * v.x()
      + self.y() * v.y()
      + self.z() * v.z()
    }

    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            &self.e[1] * v.e[2] - &self.e[2] * v.e[1],
            &self.e[2] * v.e[0] - &self.e[0] * v.e[2],
            &self.e[0] * v.e[1] - &self.e[1] * v.e[0]
        )
    }
    
    /// Rotate vector around X axis
    /// theta is in radians
    pub fn rotate_x(&self, theta: f64) -> Vec3 {
        Vec3::new(
            self.x(),
            self.y() * theta.cos() - self.z() * theta.sin(),
            self.y() * theta.sin() + self.z() * theta.cos()
        )
    }

    /// Rotate vector around Y axis
    /// theta is in radians
    pub fn rotate_y(&self, theta: f64) -> Vec3 {
        Vec3::new(
            self.x() * theta.cos() + self.z() * theta.sin(),
            self.y(),
            -self.x() * theta.sin() + self.z() * theta.cos()
        )
    }

    /// Rotate vector around Z axis
    /// theta is in radians
    pub fn rotate_z(&self, theta: f64) -> Vec3 {
        Vec3::new(
            self.x() * theta.cos() - self.y() * theta.sin(),
            self.x() * theta.sin() + self.y() * theta.cos(),
            self.z()
        )
    }

    /// Rotate vector on axis described by unit vector k
    /// (Rodrigues rotation)
    pub fn rotate_rodrigues(&self, theta: f64, _k: Vec3) -> Vec3 {
        let k;
        if _k.length() != 1.0 { k = _k.unit(); } else { k = _k; }

        *self * theta.cos() + k.cross(*self) * theta.sin() + k * (k.dot(*self) * (1.0 - theta.cos()))
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


impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(
            -self.e[0],
            -self.e[1],
            -self.e[2]
        )
    }
}

// Second part
// ===========

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2]
        )
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2]
        )
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self[0] * rhs[0],
            self[1] * rhs[1],
            self[2] * rhs[2]
        )
    }
}

// Huh, that's cool
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            rhs[0] * self,
            rhs[1] * self,
            rhs[2] * self
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs
        )
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}
