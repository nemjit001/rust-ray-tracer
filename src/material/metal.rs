use nalgebra_glm::Vec3;

use super::{Material, Scatter};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::ray_hit::RayHit;

pub struct Metal {
    albedo: Vec3,
    fuzz_factor: f32,
}

impl Metal {
    pub fn new(color: Vec3, fuzz_factor: f32) -> Self {
        let valid_fuzz = Interval::new(0.0, 1.0);

        Metal {
            albedo: color,
            fuzz_factor: valid_fuzz.clamp(fuzz_factor),
        }
    }

    fn reflect(incoming: &Vec3, normal: &Vec3) -> Vec3 {
        incoming - 2.0 * (incoming.dot(normal)) * normal
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<Scatter> {
        let scatter_direction = Self::reflect(ray.direction(), &hit.normal);
        let fuzzed_direction = scatter_direction + self.fuzz_factor * self.random_unit_vector();

        Some(Scatter {
            ray: Ray::new(hit.position, fuzzed_direction),
            attenuation: self.albedo,
        })
    }
}
