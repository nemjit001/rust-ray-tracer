pub mod radial_light;

use nalgebra_glm::Vec3;

pub trait Light {
    fn position(&self, ray_origin: &Vec3) -> Vec3;

    fn falloff_intensity(&self, distance_squared: f32) -> f32 {
        1.0 / distance_squared
    }

    fn color(&self, light_direction: &Vec3, normal: &Vec3, distance_squared: f32) -> Vec3;
}
