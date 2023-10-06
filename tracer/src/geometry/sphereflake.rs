use std::sync::Arc;
use std::f64::consts::PI;

use crate::{
    math::vec3::{Vec3, Point3},
    geometry::hittable::HittableSync,
    graphics::material::MaterialSync
};

use super::{Sphere, hittable::HittableComposite};


/// Sphere with 9 children, each with a radius 1/3 of the parent
/// Has finite volume (~1.5 times the initial sphere),
/// and inifinite surface area.  
/// 
/// **NOTE: this is not a Hittlable object.
/// Call bvh_conversion() to return a list of hittable spheres.**
pub struct Sphereflake {
    initial_sphere: Sphere,
    children: Vec<Sphereflake>
}

impl Sphereflake {
    pub fn new_upright(center: Point3, radius: f64, material: Arc<MaterialSync>, recursion_level: usize) -> Self {
        let initial = Sphere::new(center, radius, material);

        Sphereflake::new(
            initial,
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            recursion_level
        )
    }

    /// Generates a sphereflake
    /// initial is a sphere to base the rest of the spheres of
    /// normal is a Vec3 dictating the orientation of the sphereflake
    ///        the child sphere's positions are computed by rotating around the normal vector
    /// rotation_axis is the vector around which the normal is rotated to compute the children's normal
    /// recursion_level is pretty self explanatory
    pub fn new(initial: Sphere, normal: Vec3, rotation_axis: Vec3, recursion_level: usize) -> Sphereflake {
        // At level 0 it's just a regular sphere
        if recursion_level == 0 {
            return Sphereflake {
                initial_sphere: initial, 
                children: Vec::new()
            }
        }

        let child_radius = initial.radius / 3.0;
        let mut children: Vec<Sphereflake> = Vec::new();
        let initial_rot_vec = normal.unit().rotate_rodrigues(-PI / 2.0, rotation_axis) * (initial.radius + child_radius);

        let equator_theta = PI / 3.0;
        // generate 6 spheres around the equator
        for i in 0..6 {
            let child_theta = i as f64 * equator_theta;
            let child_normal = initial_rot_vec.rotate_rodrigues(child_theta, normal);

            let child_sphere = Sphere::new(
                initial.center + child_normal,
                child_radius,
                initial.material.clone()
            );

            children.push(Sphereflake::new(
                child_sphere,
                child_normal.unit(),
                rotation_axis.rotate_rodrigues(child_theta, normal),
                recursion_level - 1
            ));
        }

        // generate 3 spheres on the top with a slight initial rotation
        let initial_rotation_axis = rotation_axis.rotate_rodrigues(PI / 6.0, normal);
        let initial_rot_vec = initial_rot_vec.rotate_rodrigues(PI / 6.0, normal).rotate_rodrigues(PI / 3.5, initial_rotation_axis);
        for i in 0..3 {
            let child_theta = i as f64 * (2.0 * PI / 3.0);
            let child_normal = initial_rot_vec.rotate_rodrigues(child_theta, normal);

            let child_sphere = Sphere::new(
                initial.center + child_normal,
                child_radius,
                initial.material.clone()                
            );

            children.push(Sphereflake::new(
                child_sphere,
                child_normal.unit(),
                rotation_axis.rotate_rodrigues(child_theta, normal),
                recursion_level - 1
            ));
        }

        Sphereflake { 
            initial_sphere: initial,
            children
        }
    }
}

impl HittableComposite for Sphereflake {
    /// Recursively return a vector of all children
    fn to_hittable(&self) -> Vec<Arc<HittableSync>> {
        let mut spheres: Vec<Arc<HittableSync>> = Vec::new();
        spheres.push(Arc::new(self.initial_sphere.clone()));

        for child in &self.children {
            let mut grandsons = child.to_hittable();
            spheres.append(&mut grandsons);
        }

        spheres
    }
}
