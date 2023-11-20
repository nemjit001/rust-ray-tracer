mod camera;
mod ray;
mod interval;
mod ray_hit;
mod primitive;
mod material;
mod scene;
mod renderer;
mod timer;

use std::path::Path;
use nalgebra_glm::Vec3;

use interval::Interval;
use camera::{Resolution, Camera, FocusMode};
use primitive::{
    sphere::Sphere,
    plane::Plane,
};
use material::{diffuse::LambertianDiffuse, metal::Metal, dielectric::Dielectric};
use scene::{SkyAttenuation, Scene};
use renderer::{Renderer, RendererConfig};
use timer::Timer;

fn main() {
    println!("Raytracing in one Weekend!");

    let render_resolution = Resolution::new(1280, 720);

    let mut renderer = Renderer::new(
        &render_resolution,
        &RendererConfig {
            sample_count: 100,
            max_bounces: 25
        }
    );

    let camera = Camera::new(
        Vec3::new(0.0, 2.0, 6.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        FocusMode::AutoFocus,
        0.0,
        Interval::new(0.001, 100.0),
        &render_resolution
    );

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
                Box::new(Metal::new(Vec3::new(0.5, 0.5, 0.5), 0.2))
            )),
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

    renderer.save_render(Path::new("result.png"));

    println!("Frame time: {:?}", timer.delta_time());
}
