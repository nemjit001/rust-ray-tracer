use rand::Rng;
use nalgebra_glm::Vec3;

use crate::ray::Ray;
use crate::primitive::RayHit;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<Scatter>;
}

pub struct LambertianDiffuse {
    pub albedo: Vec3,
}

impl LambertianDiffuse {
    pub fn new(color: Vec3) -> Self {
        LambertianDiffuse {
            albedo: color,
        }
    }

    fn random_vector(min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    fn random_unit_vector() -> Vec3 {
        loop {
            let vec = Self::random_vector(-1.0, 1.0);
            if vec.magnitude_squared() < 1.0 {
                return vec.normalize()
            }
        }
    }

    fn near_zero(vector: &Vec3) -> bool {
        let s = 1e-8;

        f32::abs(vector.x) < s || f32::abs(vector.y) < s || f32::abs(vector.z) < s
    }
}

impl Material for LambertianDiffuse {
    fn scatter(&self, _ray: &Ray, hit: &RayHit) -> Option<Scatter> {
        let mut scatter_direction = hit.normal + Self::random_unit_vector();
        // If the composite vector is close to  0, use the surface normal
        if Self::near_zero(&scatter_direction) {
            scatter_direction = hit.normal
        }

        Some(Scatter {
            ray: Ray::new(hit.position, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(color: Vec3) -> Self {
        Metal {
            albedo: color,
        }
    }

    fn reflect(incoming: &Vec3, normal: &Vec3) -> Vec3 {
        incoming - 2.0 * (incoming.dot(normal)) * normal
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<Scatter> {
        let scatter_direction = Self::reflect(ray.direction(), &hit.normal);

        Some(Scatter {
            ray: Ray::new(hit.position, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
