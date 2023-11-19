use nalgebra_glm::Vec3;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::ray_hit::RayHit;
use crate::primitive::{HittablePrimitive, Hittable};

pub struct SkyAttenuation {
    pub light_color: Vec3,
    pub sky_color: Vec3,
}

pub struct Scene
{
    sky_attenuation: SkyAttenuation,
    primitives: Vec<Box<dyn HittablePrimitive>>,
}

impl Scene
{
    pub fn new(sky_attenuation: SkyAttenuation, primitives: Vec<Box<dyn HittablePrimitive>>) -> Self {
        Scene {
            sky_attenuation,
            primitives,
        }
    }

    pub fn get_sky_color(&self, ray: &Ray) -> Vec3 {
        let a = 0.5 * (ray.direction().y + 1.0);

        (1.0 - a) * self.sky_attenuation.light_color + a * self.sky_attenuation.sky_color
    }
}

impl Hittable for Scene
{
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<RayHit> {
        let mut closest_hit: Option<RayHit> = None;

        for primitive in &self.primitives {
            let closest_depth = match &closest_hit {
                Some(hit) => hit.depth,
                None => interval.max(),
            };

            if let Some(hit) = primitive.hit(ray, &Interval::new(interval.min(), closest_depth)) {
                closest_hit = match closest_hit {
                    Some(closest) => {
                        if hit.depth < closest.depth {
                            Some(hit)
                        }
                        else {
                            Some(closest)
                        }
                    },
                    None => Some(hit),
                };
            }
        }

        closest_hit
    }
}
