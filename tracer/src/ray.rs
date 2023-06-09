use crate::vec3::{Vec3, Point3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray {
    pub fn zero() -> Ray { 
        Ray { 
            origin: Vec3::origin(),
            direction: Vec3::origin()
        } 
    }
    pub fn new(origin: Point3, direction: Vec3) -> Ray { 
        Ray { 
            origin,
            direction 
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
