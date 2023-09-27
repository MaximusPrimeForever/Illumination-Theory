use std::sync::Arc;

use crate::{aabb::AABB, hittable::{HittableT, HitRecord, HittableSync}, interval::Interval, ray::Ray};

pub struct BVH {
    left_node: Arc<HittableSync>,
    right_node: Arc<HittableSync>,
    pub bounding_box: AABB
}

impl HittableT for BVH {
    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }

    /// Compute a hitrecord on a BVH tree.
    /// 
    /// First compute potential hit on the left side.
    /// If hit, then truncate the ray's interval to end at the left node
    /// Then try to hit the right side with the new interval
    /// (to see if the right side is closer than the left?)
    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord> {
        let mut bvh_hitrec: Option<HitRecord> = None;
        if !self.bounding_box.hit(ray, ray_interval) {
            return bvh_hitrec;
        }

        let right_interval: Interval;
        let left_hitrec_opt: Option<HitRecord> = self.left_node.hit(ray, ray_interval);
        match left_hitrec_opt {
            Some(left_rec) => {
                right_interval = Interval::new(ray_interval.min, left_rec.t);
                bvh_hitrec = Some(left_rec);
            }
            None => { right_interval = ray_interval; }
        }
        let right_hitrec_opt: Option<HitRecord> = self.right_node.hit(ray, right_interval);
        match right_hitrec_opt {
            Some(right_rec) => {
                bvh_hitrec = Some(right_rec);
            }
            None => {}
        }

        bvh_hitrec
    }
}


impl BVH {
    pub fn new_tree_random_axis(objects: &mut Vec<Arc<HittableSync>>) -> Self {
        BVH::new_tree_random_axis_recurse(objects, 0, objects.len())
    }

    fn new_tree_random_axis_recurse(objects: &mut Vec<Arc<HittableSync>>, start: usize, end: usize) -> Self {
        let axis_to_sort_by = rand::random::<usize>() % 3;
        let objects_span = end - start;
        let left_node: Arc<HittableSync>;
        let right_node: Arc<HittableSync>;

        let axis_comparator = |a: &Arc<HittableSync>, b: &Arc<HittableSync>| {
            a.bounding_box().axis(axis_to_sort_by).min.partial_cmp(&b.bounding_box().axis(axis_to_sort_by).min).unwrap()
        };

        match objects_span {
            1 => {
                left_node = objects[start].clone();
                right_node = objects[start].clone();
            }
            2 => {
                if axis_comparator(&objects[start], &objects[start + 1]).is_lt() {
                    left_node = objects[start].clone();
                    right_node = objects[start + 1].clone();
                } else {
                    left_node = objects[start + 1].clone();
                    right_node = objects[start].clone();
                }
            }
            _ => {
                objects[start..end].sort_by(axis_comparator);
                let mid = start + objects_span / 2;

                left_node = Arc::new(BVH::new_tree_random_axis_recurse(objects, start, mid));
                right_node = Arc::new(BVH::new_tree_random_axis_recurse(objects, mid, end));
            }
        }

        let bounding_box = AABB::new_from_aabb(
            left_node.bounding_box(),
            right_node.bounding_box()
        );
        BVH { left_node, right_node, bounding_box }
    }
}