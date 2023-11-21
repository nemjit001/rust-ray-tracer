use nalgebra_glm::Vec3;

use super::{Primitive, Hittable, HittablePrimitive};
use crate::ray_hit::RayHit;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::material::Material;

pub struct Plane {
    position: Vec3,
    normal: Vec3,
    material: Box<dyn Material>,
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3, material: Box<dyn Material>) -> Self {
        Plane {
            position,
            normal,
            material
        }
    }
}

impl Primitive for Plane {
    fn normal(&self, _location: &Vec3) -> Vec3 {
        self.normal
    }

    fn inverted_normal(&self, location: &Vec3) -> Vec3 {
        -self.normal(location)
    }

    fn material(&self) -> &dyn Material {
        self.material.as_ref()
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<RayHit> {
        let incident_angle = ray.direction().dot(&self.normal);

        if f32::abs(incident_angle) < 1e-8 {
            return None
        }

        let oc = ray.origin() - self.position;
        let depth = -oc.dot(&self.normal) / incident_angle;
        if !interval.surrounds(depth) {
            return None
        }

        let position = ray.at(depth);
        Some(RayHit::new(depth, position, ray, self))
    }
}

impl HittablePrimitive for Plane {
    //
}
