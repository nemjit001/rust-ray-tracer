use nalgebra_glm::Vec3;

use crate::ray::Ray;
use crate::primitive::Primitive;
use crate::material::Material;

#[derive(Debug)]
pub enum HitType {
    FrontFace,
    BackFace,
}

pub struct RayHit<'primitive_lifetime> {
    pub depth: f32,
    pub position: Vec3,
    pub hit_type: HitType,
    pub normal: Vec3,
    pub material: &'primitive_lifetime Box<dyn Material>,
}

impl<'a> RayHit<'a> {
    pub fn new<P>(depth: f32, position: Vec3, ray: &Ray, primitive: &'a P) -> Self
    where
        P: Primitive
    {
        let mut normal = primitive.normal(&position);
        let mut hit_type = HitType::FrontFace;

        // Dot product N * D is positive if vectors are aligned (i.e. the ray comes from inside the object!)
        if normal.dot(ray.direction()) > 0.0 {
            normal = primitive.inverted_normal(&position);
            hit_type = HitType::BackFace;
        }

        RayHit {
            depth,
            position,
            hit_type,
            normal,
            material: primitive.material(),
        }
    }
}

impl<'a> std::fmt::Debug for RayHit<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RayHit")
            .field("depth", &self.depth)
            .field("position", &self.position)
            .field("hit_type", &self.hit_type)
            .field("normal", &self.normal)
            .field("material", &"Dyn Material")
            .finish()
    }
}
