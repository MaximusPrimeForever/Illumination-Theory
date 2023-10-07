use std::sync::Arc;

use crate::{math::vec3::{Point3, Vec3}, graphics::{material::MaterialSync, bvh::BVH}};

use super::{hittable::{HittableComposite, HittableSync}, Quad};


pub fn box_new(back_bottom_left: Point3, front_upper_right: Point3, material: Arc<MaterialSync>) -> Arc<HittableSync> {
    let mut _quads = HittableComposite::new();

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
    _quads.add_hittable(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        height,
        width,
        material.clone()
    )));

    // right
    _quads.add_hittable(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        height,
        -depth,
        material.clone()
    )));

    // back
    _quads.add_hittable(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        height,
        -width,
        material.clone()
    )));

    // left
    _quads.add_hittable(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        height,
        depth,
        material.clone()
    )));

    // top
    _quads.add_hittable(Arc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        -depth,
        width,
        material.clone()
    )));

    // bottom
    _quads.add_hittable(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        -depth,
        width,
        material.clone()
    )));

    Arc::new(BVH::new(&mut _quads))
    // Arc::new(_quads)
}
