pub mod diffuse;
pub mod metal;

use rand::Rng;
use nalgebra_glm::Vec3;

use crate::ray::Ray;
use crate::ray_hit::RayHit;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<Scatter>;

    fn random_vector(&self, min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    fn random_unit_vector(&self) -> Vec3 {
        loop {
            let vec = self.random_vector(-1.0, 1.0);
            if vec.magnitude_squared() < 1.0 {
                return vec.normalize()
            }
        }
    }

    fn near_zero(&self, vector: &Vec3) -> bool {
        let s = 1e-8;

        f32::abs(vector.x) < s || f32::abs(vector.y) < s || f32::abs(vector.z) < s
    }
}
