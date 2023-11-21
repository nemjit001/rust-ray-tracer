pub mod sphere;
pub mod plane;

use nalgebra_glm::Vec3;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::ray_hit::RayHit;
use crate::material::Material;

pub trait Primitive {
    fn normal(&self, location: &Vec3) -> Vec3;

    fn inverted_normal(&self, location: &Vec3) -> Vec3;

    fn material(&self) -> &dyn Material;
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<RayHit>;
}

pub trait HittablePrimitive: Hittable + Primitive {}
