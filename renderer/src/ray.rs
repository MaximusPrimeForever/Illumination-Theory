use crate::vec3;

use vec3::Vec3 as Vec3;
use Vec3 as Point3;

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
    pub fn new(origin: &Point3, direction: &Vec3) -> Ray { 
        Ray { 
            origin: origin.clone(),
            direction: direction.clone() 
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}