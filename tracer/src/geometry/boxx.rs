use std::sync::Arc;

use crate::{math::vec3::{Point3, Vec3}, graphics::material::MaterialSync};

use super::{hittable::{HittableComposite, HittableSync}, Quad};


pub struct Boxx {
    quads: Vec<Arc<Quad>>
}

impl Boxx {
    pub fn new(back_bottom_left: Point3, front_upper_right: Point3, material: Arc<MaterialSync>) -> Self {
        let mut _quads: Vec<Arc<Quad>> = Vec::new();

        let min = Point3::new(
            back_bottom_left.x().min(front_upper_right.x()),
            back_bottom_left.y().min(front_upper_right.y()),
            back_bottom_left.z().min(front_upper_right.z()),
        );
        let max = Point3::new(
            back_bottom_left.x().max(front_upper_right.x()),
            back_bottom_left.y().max(front_upper_right.y()),
            back_bottom_left.z().max(front_upper_right.z()),
        );

        let width = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let height = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let depth = Vec3::new(0.0, 0.0, max.z() - min.z());

        // front
        _quads.push(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), max.z()),
            height,
            width,
            material.clone()
        )));

        // right
        _quads.push(Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), max.z()),
            height,
            -depth,
            material.clone()
        )));

        // back
        _quads.push(Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), min.z()),
            height,
            -width,
            material.clone()
        )));

        // left
        _quads.push(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            height,
            depth,
            material.clone()
        )));

        // top
        _quads.push(Arc::new(Quad::new(
            Point3::new(min.x(), max.y(), max.z()),
            -depth,
            width,
            material.clone()
        )));

        // bottom
        _quads.push(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), max.z()),
            -depth,
            width,
            material.clone()
        )));

        Boxx { quads: _quads }
    }
}

impl HittableComposite for Boxx {
    fn to_hittable(&self) -> Vec<std::sync::Arc<super::hittable::HittableSync>> {
        let mut sides: Vec<Arc<HittableSync>> = Vec::new();

        for side in &self.quads {
            sides.push(side.clone())
        }
        
        sides
    }
}