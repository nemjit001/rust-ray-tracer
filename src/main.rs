mod resolution;
mod camera;
mod ray;
mod interval;
mod ray_hit;
mod primitive;
mod material;
mod light;
mod scene;
mod renderer;
mod timer;

use std::path::Path;
use nalgebra_glm::Vec3;

use resolution::Resolution;
use interval::Interval;
use camera::{Camera, FocusMode};
use primitive::{
    sphere::Sphere,
    plane::{Plane, Rectangle}
};
use light::radial_light::RadialLight;
use material::{
    diffuse::LambertianDiffuse,
    metal::Metal,
    dielectric::Dielectric,
    emissive::Emissive
};
use scene::{SkyAttenuation, Scene};
use renderer::{Renderer, RendererConfig};
use timer::Timer;

fn main() {
    println!("Raytracing in one Weekend!");

    let render_resolution = Resolution::new(1920, 1080);
    let mut renderer = Renderer::new(
        RendererConfig {
            resolution: render_resolution,
            sample_count: 500,
            max_bounces: 10
        }
    );

    let camera = Camera::new(
        Vec3::new(0.0, 2.0, 6.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        FocusMode::AutoFocus,
        1.5,
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
                Box::new(LambertianDiffuse::new(Vec3::new(0.5, 0.5, 0.5)))
            )),
            Box::new(Sphere::new(
                Vec3::new(0.0, 1.0, -1.0),
                1.0,
                Box::new(LambertianDiffuse::new(Vec3::new(1.0, 0.0, 0.0)))
            )),
            Box::new(Sphere::new(
                Vec3::new(2.0, 1.0, -1.0),
                0.75,
                Box::new(Metal::new(Vec3::new(0.7, 0.5, 1.0), 0.75))
            )),
            Box::new(Sphere::new(
                Vec3::new(-2.25, 1.0, -1.0),
                1.0,
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
            Box::new(Rectangle::new(
                Vec3::new(0.0, 4.5, -2.0),
                Vec3::new(0.0, 1.0, 0.0),
                0.5, 1.0,
                Box::new(Emissive::new(Vec3::new(1.0, 0.6, 0.6), 15.0))
            )),
            Box::new(Sphere::new(
                Vec3::new(2.0, 2.0, -10.0),
                2.0,
                Box::new(LambertianDiffuse::new(Vec3::new(0.7, 0.2, 0.1)))
            )),

            Box::new(Sphere::new(
                Vec3::new(-5.0, 2.0, -10.0),
                2.0,
                Box::new(LambertianDiffuse::new(Vec3::new(0.2, 0.7, 0.1)))
            )),
        ],
        vec![
            Box::new(RadialLight::new(
                Vec3::new(0.0, 5.0, 4.0),
                Vec3::new(1.0, 1.0, 1.0),
                2.0,
                50.0,
            )),
            Box::new(RadialLight::new(
                Vec3::new(-4.0, 6.0, -3.0),
                Vec3::new(0.0, 0.3, 0.8),
                0.5,
                50.0,
            )),
            Box::new(RadialLight::new(
                Vec3::new(5.0, 8.0, -3.0),
                Vec3::new(1.0, 0.7, 0.2),
                0.5,
                50.0,
            ))
        ]
    );

    timer.tick();
    renderer.render(&camera, &scene);
    timer.tick();
    println!("Frame time: {:?} ({} FPS)", timer.delta_time(), 1.0 / timer.delta_time_f32());

    renderer.save_render(Path::new("result.png"));
}
