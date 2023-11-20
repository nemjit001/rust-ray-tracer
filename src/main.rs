mod camera;
mod ray;
mod interval;
mod ray_hit;
mod primitive;
mod material;
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
    plane::Plane,
};
use material::{diffuse::LambertianDiffuse, metal::Metal, dielectric::Dielectric};
use scene::{SkyAttenuation, Scene};
use timer::Timer;

pub struct Renderer {
    render_target: RgbImage,
    sample_count: u32,
    max_bounces: u32,
}

impl Renderer {
    pub fn new(resolution: &Resolution, sample_count: u32, max_bounces: u32) -> Self {
        let render_target: RgbImage = ImageBuffer::new(resolution.width(), resolution.height());

        Renderer {
            render_target,
            sample_count,
            max_bounces,
        }
    }

    pub fn render(&mut self, camera: &Camera, scene: &Scene) {
        let render_resolution = camera.resolution();
        let z_interval = camera.scene_depth_interval();

        for y in 0..render_resolution.height() {
            for x in 0..render_resolution.width() {
                let pixel_center = camera.get_pixel_center(x, y);

                let mut sample_sum_color = Vec3::zeros();
                for sample_num in 0..self.sample_count {
                    let pixel_sample = camera.sample_pixel(pixel_center, sample_num);

                    let ray_direction = pixel_sample - camera.position();
                    let ray_direction = ray_direction.normalize();

                    let ray = Ray::new(camera.position(), ray_direction);
                    let color = self.bounce_ray(&ray, scene, z_interval, self.max_bounces);

                    sample_sum_color += color;
                }
    
                let color = sample_sum_color / self.sample_count as f32;
                let color = Self::rgb_to_gamma(color);
                let color = Self::vec3_to_color(&color);
                self.render_target.put_pixel(x, y, color);
            }
        }

        self.render_target.save("result.png").expect("Failed to save output image");
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
                        return scatter.attenuation.component_mul(&self.bounce_ray(&scatter.ray, scene, z_interval, depth - 1));
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

fn main() {
    println!("Raytracing in one Weekend!");

    let render_resolution = Resolution::new(1280, 720);
    let camera = Camera::new(
        Vec3::new(0.0, 2.0, 6.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        Interval::new(0.001, 100.0),
        &render_resolution
    );

    let mut renderer = Renderer::new(&render_resolution, 100, 50);
    let mut timer = Timer::new();

    let scene = Scene::new(
        SkyAttenuation {
            light_color: Vec3::new(1.0, 1.0, 1.0),
            sky_color: Vec3::new(0.2, 0.7, 1.0),
        },
        vec![
            Box::new(Plane::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Box::new(LambertianDiffuse::new(Vec3::new(0.2, 0.2, 0.3)))
            )),  // Ground
            // Primitives below
            Box::new(Sphere::new(
                Vec3::new( 0.0, 1.0, -1.0),
                1.0,
                Box::new(LambertianDiffuse::new(Vec3::new(1.0, 0.0, 0.0)))
            )),
            Box::new(Sphere::new(
                Vec3::new( 2.0, 1.0, -1.0),
                0.75,
                Box::new(Metal::new(Vec3::new(0.7, 0.5, 1.0), 0.75))
            )),
            Box::new(Sphere::new(
                Vec3::new(-2.0, 1.0, -1.0),
                0.75,
                Box::new(Metal::new(Vec3::new(0.8, 1.0, 0.2), 0.01))
            )),
            Box::new(Sphere::new(
                Vec3::new(-1.0, 0.5, 0.5),
                0.5,
                Box::new(Dielectric::new(Vec3::new(0.2, 0.5, 0.8), 1.52))
            )),
            Box::new(Sphere::new(
                Vec3::new(-1.0, 0.5, 0.5),
                -0.4,
                Box::new(Dielectric::new(Vec3::new(0.2, 0.5, 0.8), 1.52))
            )), // Inverted sphere to model hollow inside
            Box::new(Sphere::new(
                Vec3::new(1.0, 0.25, 0.0),
                0.25,
                Box::new(Dielectric::new(Vec3::new(1.0, 0.5, 0.8), 1.77))
            )),
            Box::new(Sphere::new(
                Vec3::new(0.0, 0.25, 1.0),
                0.25,
                Box::new(Dielectric::new(Vec3::new(0.2, 1.0, 1.0), 2.17))
            )),
        ]
    );

    renderer.render(&camera, &scene);
    timer.tick();

    println!("Frame time: {:?}", timer.delta_time());
}
