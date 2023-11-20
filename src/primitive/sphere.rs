use nalgebra_glm::Vec3;

use super::{Primitive, Hittable, HittablePrimitive};
use crate::ray_hit::RayHit;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere {
    position: Vec3,
    radius: f32,
    radius_squared: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f32, material: Box<dyn Material>) -> Self {
        Sphere {
            position,
            radius,
            radius_squared: radius * radius,
            material,
        }
    }
}

impl Primitive for Sphere {
    fn normal(&self, location: &Vec3) -> Vec3 {
        (location - self.position) / self.radius
    }

    fn inverted_normal(&self, location: &Vec3) -> Vec3 {
        -self.normal(location)
    }

    fn material(&self) -> &Box<dyn Material> {
        &self.material
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<RayHit> {
        let oc = ray.origin() - self.position;
        let a = ray.direction().magnitude_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.magnitude_squared() - self.radius_squared;

        let disciminant = (half_b * half_b) - (a * c);
        if disciminant < 0.0 {
            return None
        }

        // Beware: Expensive calculations below!
        let mut depth = (-half_b - f32::sqrt(disciminant)) / a;
        if !interval.surrounds(depth) {
            // Check if ray comes from inside the sphere (Note the '+' instead of '-')
            depth = (-half_b + f32::sqrt(disciminant)) / a;
            if !interval.surrounds(depth)
            {
                return None
            }
        }

        let position = ray.at(depth);
        Some(RayHit::new(depth, position, ray, self))
    }
}

impl HittablePrimitive for Sphere {}
