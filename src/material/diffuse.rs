use nalgebra_glm::Vec3;

use super::{Material, Scatter};
use crate::ray::Ray;
use crate::ray_hit::RayHit;

pub struct LambertianDiffuse {
    pub albedo: Vec3,
}

impl LambertianDiffuse {
    pub fn new(color: Vec3) -> Self {
        LambertianDiffuse {
            albedo: color,
        }
    }
}

impl Material for LambertianDiffuse {
    fn scatter(&self, _ray: &Ray, hit: &RayHit) -> Option<Scatter> {
        let mut scatter_direction = hit.normal + self.random_unit_vector();
        // If the composite vector is close to  0, use the surface normal
        if self.near_zero(&scatter_direction) {
            scatter_direction = hit.normal
        }

        Some(Scatter {
            ray: Ray::new(hit.position, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
