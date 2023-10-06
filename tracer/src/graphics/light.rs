use std::sync::Arc;

use crate::{Point3, Color};

use super::{
    texture::{TextureSync, SolidColorTexture},
    material::Material
};

/// My own implementation of a point light.
/// It does not account for distance at all.
/// Camera::ray_color() should be modified with a call to World::hit_lights()
/// to use it.
pub struct Light {
    pub origin: Point3,
    pub color: Color,
    pub brightness: f64
}

impl Light {
    pub fn new(origin: Point3, color: Color, brightness: f64) -> Light {
        Light { origin, color, brightness }
    }
}

// ==============================

pub struct DiffuseLight {
    emit: Arc<TextureSync>
}

impl DiffuseLight {
    pub fn new(texture: Arc<TextureSync>) -> Self {
        DiffuseLight { emit: texture }
    }

    pub fn new_color(color: Color) -> Self {
        DiffuseLight { emit: Arc::new(SolidColorTexture::new(color)) }
    }


}

impl Material for DiffuseLight {
    fn scatter(&self, _: &crate::ray::Ray, _: &crate::hittable::HitRecord) -> Option<(Color, crate::ray::Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, point: &Point3) -> Color {
        self.emit.value(u, v, point)
    }
}