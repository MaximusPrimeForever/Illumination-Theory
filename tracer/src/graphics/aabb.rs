use std::sync::Arc;

use crate::geometry::{Ray, hittable::HittableSync};
use crate::math::{vec3::{Point3, Vec3}, interval::Interval};


#[derive(Default, Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval
}

impl AABB {
    pub fn new(_x: Interval, _y: Interval, _z: Interval) -> AABB {
        AABB { x: _x, y: _y, z: _z }
    }

    /// Treat the two points a and b as extrema for the bounding box, so we don't require a
    /// particular minimum/maximum coordinate order.
    pub fn new_from_points(a: Point3, b: Point3) -> AABB {
        AABB { 
            x: Interval::new(a[0].min(b[0]), a[0].max(b[0])),
            y: Interval::new(a[1].min(b[1]), a[1].max(b[1])),
            z: Interval::new(a[2].min(b[2]), a[2].max(b[2])),
        }
    }

    pub fn pad(&self) -> AABB {
        let delta = 0.0001;

        let new_x = if self.x.len() >= delta { self.x.clone() } else { self.x.expand(delta) };
        let new_y = if self.y.len() >= delta { self.y.clone() } else { self.y.expand(delta) };
        let new_z = if self.z.len() >= delta { self.z.clone() } else { self.z.expand(delta) };

        AABB::new(new_x, new_y, new_z)
    }

    pub fn new_from_hittables(objects: &Vec<Arc<HittableSync>>) -> AABB {
        let mut bounding_box = AABB::default();
        for obj in objects {
            bounding_box += obj.bounding_box();
        }

        bounding_box
    }

    pub fn axis(&self, n: usize) -> Interval {
        match n {
            0 => { self.x },
            1 => { self.y },
            2 => { self.z },
            _ => { panic!("Invalid axis index.") }
        }
    }
    
    pub fn hit(&self, ray: Ray, mut ray_interval: Interval) -> bool {
        let mut dimension_interval;

        for i in 0..3 {
            let min_intersection = (self.axis(i).min - ray.origin[i]) / ray.direction[i];
            let max_intersection = (self.axis(i).max - ray.origin[i]) / ray.direction[i];

            // Make sure current dimension interval is correctly ordered
            dimension_interval = Interval::new(
                min_intersection.min(max_intersection),
                min_intersection.max(max_intersection)
            );

            // Intersect ray interval with current dimension
            // if an intersection exists then the ray interval shrinks to that
            // interval.
            ray_interval = ray_interval.intersect(dimension_interval);
            if ray_interval.is_empty() {
                return false;
            }
        }

        true
    }
}

impl std::ops::Add<AABB> for AABB {
    type Output = AABB;

    fn add(self, rhs: AABB) -> Self::Output {
        AABB { 
            x: self.x.unite(rhs.x),
            y: self.y.unite(rhs.y),
            z: self.z.unite(rhs.z),
        }
    }
}

impl std::ops::AddAssign<AABB> for AABB {
    fn add_assign(&mut self, rhs: AABB) {
        *self = *self + rhs;
    }
}

impl std::ops::Add<Vec3> for AABB {
    type Output = AABB;

    fn add(self, rhs: Vec3) -> Self::Output {
        AABB::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl std::ops::Add<AABB> for Vec3 {
    type Output = AABB;

    fn add(self, rhs: AABB) -> Self::Output {
        AABB::new(self.x() + rhs.x, self.y() + rhs.y, self.z() + rhs.z)
    }
}
