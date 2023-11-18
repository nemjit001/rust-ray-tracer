use super::interval::Interval;
use super::ray::Ray;
use super::primitive::{HittablePrimitive, Hittable, RayHit};

pub struct Scene
{
    primitives: Vec<Box<dyn HittablePrimitive>>,
}

impl Scene
{
    pub fn new(primitives: Vec<Box<dyn HittablePrimitive>>) -> Self {
        Scene {
            primitives
        }
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
