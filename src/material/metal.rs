use nalgebra_glm::Vec3;

use super::{Material, Scatter};
use crate::ray::Ray;
use crate::ray_hit::RayHit;

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(color: Vec3) -> Self {
        Metal {
            albedo: color,
        }
    }

    fn reflect(incoming: &Vec3, normal: &Vec3) -> Vec3 {
        incoming - 2.0 * (incoming.dot(normal)) * normal
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<Scatter> {
        let scatter_direction = Self::reflect(ray.direction(), &hit.normal);

        Some(Scatter {
            ray: Ray::new(hit.position, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
