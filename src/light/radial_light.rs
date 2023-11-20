use rand::Rng;
use nalgebra_glm::Vec3;

use super::Light;

pub struct RadialLight {
    position: Vec3,
    color: Vec3,
    radius: f32,
    base_intensity: f32,
}

impl RadialLight {
    pub fn new(position: Vec3, color: Vec3, radius: f32, base_intensity: f32) -> Self {
        RadialLight {
            position,
            color,
            radius,
            base_intensity,
        }
    }

    fn random_vector(&self, min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max)
        )
    }

    fn sample_light_sphere(&self) -> Vec3 {
        loop {
            let vec = self.random_vector(-1.0, 1.0);

            if vec.magnitude_squared() < 1.0 {
                return vec * self.radius;
            }
        }
    }
}

impl Light for RadialLight {
    fn position(&self) -> Vec3 {
        self.position + self.sample_light_sphere()
    }

    fn color(&self, incident_angle: f32, distance_squared: f32) -> Vec3 {
        incident_angle * self.base_intensity * self.color * self.falloff_intensity(distance_squared)
    }
}
