use nalgebra_glm::Vec3;
use rand::Rng;

use super::{Material, Scatter};
use crate::ray::Ray;
use crate::ray_hit::{RayHit, HitType};


pub struct Dielectric {
    albedo: Vec3,
    index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(color: Vec3, index_of_refraction: f32) -> Self {
        Dielectric {
            albedo: color,
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<Scatter> {
        let ior_fraction = match hit.hit_type {
            HitType::FrontFace => 1.0 / self.index_of_refraction,
            HitType::BackFace => self.index_of_refraction / 1.0,
        };

        let cos_theta = f32::min(-1.0 * ray.direction().dot(&hit.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - (cos_theta * cos_theta));
        let must_reflect = (ior_fraction * sin_theta) > 1.0;

        let mut rng = rand::thread_rng();
        let reflect_chance = rng.gen_range(0.0..1.0);
        let reflectance = self.reflectance(cos_theta, ior_fraction);

        let ray_direction = if must_reflect || reflectance > reflect_chance {
            self.reflect(ray.direction(), &hit.normal)
        }
        else {
            self.refract(ray.direction(), &hit.normal, ior_fraction)
        };

        Some(Scatter {
            ray: Ray::new(hit.position, ray_direction),
            attenuation: self.albedo,
        })
    }
}
