use crate::vec3::{Vec3, Point3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64
}

impl Ray {
    #[allow(dead_code)]
    pub fn zero() -> Ray { 
        Ray { 
            origin: Vec3::zero(),
            direction: Vec3::zero(),
            time: 0.0
        }
    }
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Ray { 
        Ray { 
            origin,
            direction,
            time
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
