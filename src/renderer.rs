mod gl_layer;

use std::path::Path;
use nalgebra_glm::Vec3;
use image::{RgbImage, ImageBuffer, Rgb};

use gl_layer::{
    vertex_array::VertexArray,
    shaders::{ShaderType, Shader, ShaderPipeline}
};

use crate::resolution::Resolution;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::camera::Camera;
use crate::primitive::Hittable;
use crate::scene::Scene;

#[derive(Clone, Copy)]
pub struct RendererConfig {
    pub resolution: Resolution,
    pub sample_count: u32,
    pub max_bounces: u32,
}

struct RasterPass {
    vertex_array_object: VertexArray,
    pipeline: ShaderPipeline,
}

impl RasterPass {
    pub fn new() -> Self {
        let vertex_shader = Shader::new(
            ShaderType::Vertex,
            r#"
            #version 450

            out vec2 screen_uv;

            void main() {
                screen_uv = vec2((gl_VertexID << 1) & 2, gl_VertexID & 2);
                gl_Position = vec4(screen_uv * 2.0f + -1.0f, 0.0f, 1.0f);
            }
            "#
        );

        let fragment_shader = Shader::new(
            ShaderType::Fragment,
            r#"
            #version 450

            in vec2 screen_uv;
            out vec4 frag_color;

            void main() {
                frag_color = vec4(screen_uv, 0, 1);
            }
            "#
        );

        let vertex_array_object = VertexArray::new();
        let pipeline = ShaderPipeline::new(&[vertex_shader, fragment_shader]);

        RasterPass {
            vertex_array_object,
            pipeline
        }
    }

    pub fn execute(&self) {
        unsafe {
            gl::Clear(gl::COLOR);
        }

        // TODO: bind texture for use
        self.vertex_array_object.bind();
        self.pipeline.bind();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

pub struct Renderer {
    config: RendererConfig,
    render_target: RgbImage,
    raster_pass: RasterPass,
}

impl Renderer {
    pub fn new(window: &mut glfw::Window, config: &RendererConfig) -> Self {
        gl_layer::init(window);

        let render_target: RgbImage = ImageBuffer::new(config.resolution.width(), config.resolution.height());
        let raster_pass = RasterPass::new();

        Renderer {
            config: *config,
            render_target,
            raster_pass,
        }
    }

    pub fn render(&mut self, camera: &Camera, scene: &Scene) {
        let z_interval = camera.scene_depth_interval();

        for y in 0..self.config.resolution.height() {
            for x in 0..self.config.resolution.width() {
                let mut sample_sum_color = Vec3::zeros();

                for sample in 0..self.config.sample_count {
                    let ray = self.get_ray(camera, x, y, sample);
                    let color = self.bounce_ray(&ray, scene, z_interval, self.config.max_bounces);

                    sample_sum_color += color;
                }
    
                let color = sample_sum_color / self.config.sample_count as f32;
                let color = Self::rgb_to_gamma(color);
                let color = Self::vec3_to_color(&color);
                self.render_target.put_pixel(x, y, color);
            }
        }

        self.raster_pass.execute();
    }

    pub fn save_render(&self, path: &Path) {
        self.render_target.save(path).expect("Failed to save output image");
    }

    fn get_ray(&self, camera: &Camera, x: u32, y: u32, sample: u32) -> Ray {
        let pixel_center = camera.get_pixel_center(x, y);
        let pixel_sample = camera.sample_pixel(pixel_center, sample);

        let ray_origin = camera.get_ray_origin();
        let ray_direction = pixel_sample - ray_origin;
        let ray_direction = ray_direction.normalize();

        Ray::new(ray_origin, ray_direction)
    }

    fn bounce_ray(&self, ray: &Ray, scene: &Scene, z_interval: &Interval, depth: u32) -> Vec3 {
        if depth == 0 {
            return Vec3::zeros();
        }

        let closest_hit = scene.hit(ray, z_interval);
        match closest_hit {
            Some(hit) => {
                let scatter = hit.material.scatter(ray, &hit);

                match scatter {
                    Some(scatter) => {
                        let object_color = scatter.attenuation.component_mul(&self.bounce_ray(&scatter.ray, scene, z_interval, depth - 1));
                        let light_color = scene.shadow_ray(&hit, z_interval);

                        return object_color.component_mul(&light_color)
                    },
                    None => return Vec3::zeros(),
                }
            }
            None => {
                return scene.get_sky_color(&ray);
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

    fn vec3_to_color(color: &Vec3) -> Rgb<u8> {
        let intensity = Interval::new(0.0, 0.999);

        Rgb([
            (intensity.clamp(color.x) * 256.0) as u8,
            (intensity.clamp(color.y) * 256.0) as u8,
            (intensity.clamp(color.z) * 256.0) as u8,
        ])
    }
}
