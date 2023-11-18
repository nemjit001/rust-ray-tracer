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

pub trait HittablePrimitive: Hittable + Primitive {
    //
}

pub struct Sphere {
    position: Vec3,
    radius: f32,
    radius_squared: f32
}

impl Sphere {
    pub fn new(position: Vec3, radius: f32) -> Self {
        Sphere {
            position,
            radius,
            radius_squared: radius * radius
        }
    }

    pub fn position(&self) -> &Vec3 {
        &self.position
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Primitive for Sphere {
    fn normal(&self, location: &Vec3) -> Vec3 {
        (location - self.position) / self.radius
    }

    fn inverted_normal(&self, location: &Vec3) -> Vec3 {
        -self.normal(location)
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
            // Check if ray comes from inside the sphere
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

impl HittablePrimitive for Sphere {
    // XXX: Empty combined trait
}
