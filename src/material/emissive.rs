use nalgebra_glm::Vec3;

use super::{Material, Scatter};
use crate::ray::Ray;
use crate::ray_hit::RayHit;

pub struct Emissive {
    pub color: Vec3,
    pub strength: f32
}

impl Emissive {
    pub fn new(color: Vec3, strength: f32) -> Self {
        Emissive {
            color,
            strength,
        }
    }
}

impl Material for Emissive {
    fn scatter(&self, _ray: &Ray, _hit: &RayHit) -> Option<Scatter> {
        None
    }

    fn emit(&self) -> Vec3 {
        self.color * self.strength
    }
}
