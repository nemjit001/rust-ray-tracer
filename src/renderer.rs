use std::path::Path;
use nalgebra_glm::Vec3;
use image::{RgbImage, Rgb};

use crate::resolution::Resolution;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::camera::Camera;
use crate::primitive::Hittable;
use crate::material::MaterialTransparency;
use crate::scene::Scene;

#[derive(Clone, Copy)]
pub struct RendererConfig {
    pub resolution: Resolution,
    pub sample_count: u32,
    pub max_bounces: u32,
}

pub struct Renderer {
    config: RendererConfig,
    render_target: RgbImage,
}

impl Renderer {
    pub fn new(config: RendererConfig) -> Self {
        Renderer {
            config,
            render_target: RgbImage::new(config.resolution.width(), config.resolution.height()),
        }
    }

    #[cfg(feature = "parallel")]
    pub fn render(&mut self, camera: &Camera, scene: &Scene) {
        use rayon::prelude::*;

        let z_interval = camera.scene_depth_interval();
        let (width, height) = self.config.resolution.dimensions();

        let one_over_sample_count = 1.0 / self.config.sample_count as f32;
        let render_output = std::sync::Mutex::new(&mut self.render_target);
        (0..height).into_par_iter().for_each(|y| {
            for x in 0..width {
                let mut sample_color = Vec3::zeros();

                for sample in 0..self.config.sample_count {
                    let ray = camera.get_primary_ray(x, y, sample);
                    let color = Self::bounce_ray(&ray, scene, z_interval, self.config.max_bounces);

                    sample_color += color;
                }
    
                let color = Self::rgb_to_gamma(sample_color * one_over_sample_count);
                let color = Self::vec3_to_color(color);
                
                render_output.lock().unwrap().put_pixel(x, y, color);
            }
        })
    }

    #[cfg(feature = "single_threaded")]
    pub fn render(&mut self, camera: &Camera, scene: &Scene) {
        let z_interval = camera.scene_depth_interval();
        let (width, height) = self.config.resolution.dimensions();

        for y in 0..height {
            for x in 0..width {
                let mut sample_sum_color = Vec3::zeros();

                for sample in 0..self.config.sample_count {
                    let ray = camera.get_primary_ray(x, y, sample);
                    let color = Self::bounce_ray(&ray, scene, z_interval, self.config.max_bounces);

                    sample_sum_color += color;
                }
    
                let color = sample_sum_color / self.config.sample_count as f32;
                let color = Self::rgb_to_gamma(color);
                let color = Self::vec3_to_color(color);
                self.render_target.put_pixel(x, y, color);
            }
        }
    }

    pub fn save_render(&self, path: &Path) {
        self.render_target.save(path).expect("Failed to save output image");
    }

    fn bounce_ray(ray: &Ray, scene: &Scene, z_interval: &Interval, depth: u32) -> Vec3 {
        if depth == 0 {
            return Vec3::zeros();
        }

        let closest_hit = scene.hit(ray, z_interval);
        match closest_hit {
            Some(hit) => {
                let scatter = hit.material.scatter(ray, &hit);

                match scatter {
                    Some(scatter) => {
                        let object_color = scatter.attenuation.component_mul(
                            &Self::bounce_ray(&scatter.ray, scene, z_interval, depth - 1)
                        );

                        match hit.material.material_transparency() {
                            MaterialTransparency::Opaque => object_color.component_mul(&scene.shadow_ray(&hit, z_interval)),
                            MaterialTransparency::Transparent => object_color,
                        }
                    },
                    None => {
                        hit.material.emit()
                    },
                }
            }
            None => {
                scene.get_sky_color(ray)
            }
        }
    }

    fn rgb_to_gamma(color: Vec3) -> Vec3 {
        Vec3::new(
            f32::sqrt(color.x),
            f32::sqrt(color.y),
            f32::sqrt(color.z),
        )
    }

    fn vec3_to_color(color: Vec3) -> Rgb<u8> {
        let intensity = Interval::new(0.0, 0.999);

        Rgb([
            (intensity.clamp(color.x) * 256.0) as u8,
            (intensity.clamp(color.y) * 256.0) as u8,
            (intensity.clamp(color.z) * 256.0) as u8,
        ])
    }
}
