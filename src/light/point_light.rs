use nalgebra_glm::Vec3;

use super::Light;

#[deprecated]
pub struct PointLight {
    position: Vec3,
    color: Vec3,
    base_intensity: f32,
}

impl PointLight {
    pub fn new(position: Vec3, color: Vec3, base_intensity: f32) -> Self {
        PointLight {
            position,
            color,
            base_intensity,
        }
    }
}

impl Light for PointLight {
    fn position(&self) -> Vec3 {
        self.position
    }

    fn color(&self, incident_angle: f32, distance_squared: f32) -> Vec3 {
        incident_angle * self.base_intensity * self.color * self.falloff_intensity(distance_squared)
    }
}
