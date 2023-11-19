use nalgebra_glm::Vec3;

use crate::ray::Ray;
use crate::primitive::Primitive;
use crate::material::Material;

pub struct RayHit<'primitive_lifetime> {
    pub depth: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: &'primitive_lifetime Box<dyn Material>,
}

impl<'a> RayHit<'a> {
    pub fn new<P>(depth: f32, position: Vec3, ray: &Ray, primitive: &'a P) -> Self
    where
        P: Primitive
    {
        let mut normal = primitive.normal(&position);

        // Dot product N * D is positive if vectors are aligned (i.e. the ray comes from inside the object!)
        if normal.dot(ray.direction()) > 0.0 {
            normal = primitive.inverted_normal(&position);
        }

        RayHit {
            depth,
            position,
            normal,
            material: primitive.material(),
        }
    }
}
