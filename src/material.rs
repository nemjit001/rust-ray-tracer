pub mod diffuse;
pub mod metal;
pub mod dielectric;

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

    fn reflect(&self, incoming: &Vec3, normal: &Vec3) -> Vec3 {
        incoming - 2.0 * (incoming.dot(normal)) * normal
    }

    fn refract(&self, incoming: &Vec3, normal: &Vec3, ior_fraction: f32) -> Vec3 {
        let cos_theta = f32::min(-incoming.dot(normal), 1.0);

        let out_perpendicular = ior_fraction * (incoming + cos_theta * normal);
        let out_parallel = -1.0 * f32::sqrt(f32::abs(1.0 - out_perpendicular.magnitude_squared())) * normal;

        out_perpendicular + out_parallel
    }

    fn reflectance(&self, cos_angle: f32, ior_fraction: f32) -> f32 {
        // Schlick's approximation for reflectance
        let r0 = (1.0 - ior_fraction) / (1.0 + ior_fraction);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cos_angle).powi(5)
    }
}
