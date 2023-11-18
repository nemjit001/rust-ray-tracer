mod camera;
mod ray;
mod interval;
mod primitive;
mod scene;
mod timer;

use image::{RgbImage, ImageBuffer, Rgb};
use nalgebra_glm::Vec3;

use ray::Ray;
use interval::Interval;
use camera::{Resolution, Camera};
use primitive::{
    Hittable,
    sphere::Sphere,
};
use scene::Scene;
use timer::Timer;

pub struct Renderer {
    render_target: RgbImage,
    sample_count: u32,
}

impl Renderer {
    pub fn new(resolution: &Resolution, sample_count: u32) -> Self {
        let render_target: RgbImage = ImageBuffer::new(resolution.width(), resolution.height());

        Renderer {
            render_target,
            sample_count,
        }
    }

    pub fn render(&mut self, camera: &Camera, scene: &Scene) {
        let render_resolution = camera.resolution();
        let z_interval = camera.scene_depth_interval();

        for y in 0..render_resolution.height() {
            for x in 0..render_resolution.width() {
                let pixel_center = camera.get_pixel_center(x, y);

                let mut color_samples = vec![];
                color_samples.reserve(self.sample_count as usize);

                for sample_num in 0..self.sample_count {
                    let pixel_sample = camera.sample_pixel(pixel_center, sample_num);

                    let ray_direction = pixel_sample - camera.position();
                    let ray_direction = ray_direction.normalize();
                    let ray = Ray::new(camera.position(), ray_direction);
        
                    let mut color = Vec3::new(0.0, 0.0, 0.0);
        
                    let closest_hit = scene.hit(&ray, z_interval);
                    if let Some(hit) = closest_hit {
                        color = 0.5 * (hit.normal + Vec3::new(1.0, 1.0, 1.0));
                        // color = hit.position;

                        // let depth = (hit.depth + z_interval.min()) / z_interval.max();
                        // color = Vec3::new(depth, depth, depth);
                    }

                    color_samples.push(color);
                }
    
                let color = samples_to_color(&color_samples);
                self.render_target.put_pixel(x, y, color);
            }
        }

        self.render_target.save("result.png").expect("Failed to save output image");
    }
}

fn vec3_to_color(color: &Vec3) -> Rgb<u8> {
    Rgb([
        (color.x * 255.99) as u8,
        (color.y * 255.99) as u8,
        (color.z * 255.99) as u8,
    ])
}

fn samples_to_color(samples: &[Vec3]) -> Rgb<u8> {
    let mut sum = Vec3::zeros();

    for sample in samples {
        sum += sample;
    }

    sum /= samples.len() as f32;
    vec3_to_color(&sum)
}

fn main() {
    println!("Raytracing in one Weekend!");

    let render_resolution = Resolution::new(1280, 720);
    let mut renderer = Renderer::new(&render_resolution, 4);
    let camera = Camera::new(
        Vec3::new(0.0, 1.0, 4.0),
        2.0,
        Interval::new(0.1, 100.0),
        &render_resolution
    );
    let mut timer = Timer::new();

    let scene = Scene::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 50.0)),      // Sky dome
        Box::new(Sphere::new(Vec3::new(0.0, -100.0, 0.0), 100.0)), // Ground
        // Primitives below
        Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0)),
        Box::new(Sphere::new(Vec3::new(3.0, 1.0, -1.0), 1.0)),
        Box::new(Sphere::new(Vec3::new(-2.0, 1.0, 0.0), 0.5)),
    ]);

    renderer.render(&camera, &scene);
    timer.tick();

    println!("Frame time: {:?}", timer.delta_time());
}
