use nalgebra_glm::Vec3;

use super::{Primitive, Hittable, HittablePrimitive};
use crate::ray_hit::RayHit;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::material::Material;

pub struct Plane {
    position: Vec3,
    normal: Vec3,
    material: Box<dyn Material + Sync>,
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3, material: Box<dyn Material + Sync>) -> Self {
        Plane {
            position,
            normal,
            material,
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

pub struct Rectangle {
    position: Vec3,
    normal: Vec3,
    basis_vectors: (Vec3, Vec3),
    material: Box<dyn Material + Sync>,
}

impl Rectangle {
    pub fn new(position: Vec3, normal: Vec3, width: f32, height: f32, material: Box<dyn Material + Sync>) -> Self {
        Rectangle {
            position,
            normal,
            basis_vectors: (Vec3::new(1.0, 0.0, 0.0) * width, Vec3::new(0.0, 0.0, 1.0) * height),
            material,
        }
    }
}

impl Primitive for Rectangle {
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

impl Hittable for Rectangle {
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

        let hit_position = ray.at(depth);
        let planar_hit = self.position - hit_position;

        if f32::abs(planar_hit.dot(&self.basis_vectors.0)) > 1.0 || f32::abs(planar_hit.dot(&self.basis_vectors.1)) > 1.0 {
            return None
        }

        let position = hit_position;
        Some(RayHit::new(depth, position, ray, self))
    }
}

impl HittablePrimitive for Rectangle {
    //
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::diffuse::LambertianDiffuse;

    #[test]
    fn test_intersect() {
        let plane = Plane::new(
            Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0),
            Box::new(LambertianDiffuse::new(Vec3::new(0.0, 0.0, 0.0)))
        );

        let ray = Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        assert!(plane.hit(&ray, &Interval::new(0.01, f32::MAX)).is_some())
    }

    #[test]
    fn test_intersect_below() {
        let plane = Plane::new(
            Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0),
            Box::new(LambertianDiffuse::new(Vec3::new(0.0, 0.0, 0.0)))
        );

        let ray = Ray::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        assert!(plane.hit(&ray, &Interval::new(0.01, f32::MAX)).is_some())
    }

    #[test]
    fn test_miss() {
        let plane = Plane::new(
            Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0),
            Box::new(LambertianDiffuse::new(Vec3::new(0.0, 0.0, 0.0)))
        );

        let ray = Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        assert!(plane.hit(&ray, &Interval::new(0.01, f32::MAX)).is_none())
    }
}
