pub mod sphere;

use nalgebra_glm::Vec3;

use super::interval::Interval;
use super::ray::Ray;

#[derive(Debug)]
pub struct RayHit {
    pub depth: f32,
    pub position: Vec3,
    pub normal: Vec3,
}

impl RayHit {
    pub fn new<P>(depth: f32, position: Vec3, ray: &Ray, primitive: &P) -> Self
    where
        P: Primitive
    {
        let mut normal = primitive.normal(&position);

        // Dot product N * D is positive if vectors are aligned (i.e. the ray comes from inside the object!)
        let hit_from_inside = normal.dot(ray.direction()) > 0.0;
        if hit_from_inside {
            normal = primitive.inverted_normal(&position);
        }

        RayHit {
            depth,
            position,
            normal
        }
    }
}

pub trait Primitive {
    fn normal(&self, location: &Vec3) -> Vec3;

    fn inverted_normal(&self, location: &Vec3) -> Vec3;
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<RayHit>;
}

pub trait HittablePrimitive: Hittable + Primitive {}
