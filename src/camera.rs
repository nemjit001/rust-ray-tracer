use nalgebra_glm::Vec3;
use rand::{thread_rng, Rng};

use super::interval::Interval;

// const WORLD_FORWARD: Vec3   = Vec3::new(0.0, 0.0, 1.0);
// const WORLD_RIGHT: Vec3     = Vec3::new(1.0, 0.0, 0.0);
const WORLD_UP: Vec3        = Vec3::new(0.0, 1.0, 0.0);

#[derive(Debug, Clone, Copy)]
pub struct Resolution(u32, u32);

impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Resolution(width, height)
    }
    
    pub fn width(&self) -> u32 {
        self.0
    }

    pub fn height(&self) -> u32 {
        self.1
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width() as f32 / self.height() as f32
    }
}

#[derive(Debug)]
struct PixelDelta(Vec3, Vec3);

impl PixelDelta {
    pub fn new(delta_u: Vec3, delta_v: Vec3) -> Self {
        PixelDelta(delta_u, delta_v)
    }

    pub fn u(&self) -> Vec3 {
        self.0
    }

    pub fn v(&self) -> Vec3 {
        self.1
    }
}

#[derive(Debug)]
struct ViewPlane {
    viewport_top_left: Vec3,
    pixel_delta: PixelDelta,
}

impl ViewPlane {
    pub fn new(position: &Vec3, focal_length: f32, camera_vectors: &CameraVectors, viewport_width: f32, viewport_height: f32, resolution: &Resolution) -> Self {
        let vec_u = viewport_width * camera_vectors.up();
        let vec_v = viewport_height * -camera_vectors.right();

        let pixel_delta_u = vec_u / resolution.width() as f32;
        let pixel_delta_v = vec_v / resolution.height() as f32;

        // XXX: using z forward coordinates, meaning forward * focal length is the rotation of the camera.
        //      rotating the forward vector & calculating new plane position will give camera rot.
        let viewport_top_left = position - (focal_length * camera_vectors.forward()) - (vec_u / 2.0) - (vec_v / 2.0);

        ViewPlane {
            viewport_top_left,
            pixel_delta: PixelDelta::new(pixel_delta_u, pixel_delta_v),
        }
    }

    pub fn get_pixel_center(&self, x: f32, y: f32) -> Vec3 {
        self.viewport_top_left + self.get_pixel_offset(x, y)
    }

    pub fn get_pixel_offset(&self, x: f32, y: f32) -> Vec3 {
        x * self.pixel_delta.u() + y * self.pixel_delta.v()
    }
}

struct CameraVectors {
    forward: Vec3,
    up: Vec3,
}

impl CameraVectors {
    pub fn forward(&self) -> Vec3 {
        self.forward
    }

    pub fn up(&self) -> Vec3 {
        self.up
    }

    pub fn right(&self) -> Vec3 {
        self.forward.cross(&self.up)
    }
}

pub struct Camera {
    position: Vec3,
    scene_depth: Interval,
    resolution: Resolution,
    view_plane: ViewPlane,
    _camera_vectors: CameraVectors,
}

impl Camera {
    pub fn new(position: Vec3, look_at: Vec3, vertical_fov: f32, scene_depth: Interval, resolution: &Resolution) -> Self {
        let focal_length = (look_at - position).magnitude();
        let (viewport_width, viewport_height) = Self::calculate_viewport_extent(vertical_fov, focal_length, resolution);

        let forward = (position - look_at).normalize();
        let up = WORLD_UP.cross(&forward);
        let camera_vectors = CameraVectors {
            forward,
            up,
        };
        
        let view_plane = ViewPlane::new(&position, focal_length, &camera_vectors, viewport_width, viewport_height, resolution);
        let resolution = *resolution;

        Camera {
            position,
            scene_depth,
            resolution,
            view_plane,
            _camera_vectors: camera_vectors,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn scene_depth_interval(&self) -> &Interval {
        &self.scene_depth
    }

    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    pub fn get_pixel_center(&self, x: u32, y: u32) -> Vec3 {
        self.view_plane.get_pixel_center(x as f32, y as f32)
    }

    pub fn sample_pixel(&self, pixel_center: Vec3, _sample_num: u32) -> Vec3 {
        // Sampling is random for now -> use AA sample grid for consistent sampling
        let mut rng = thread_rng();

        let x_offset = -0.5 + rng.gen_range(0.0..1.0);
        let y_offset = -0.5 + rng.gen_range(0.0..1.0);

        let sample_offset = self.view_plane.get_pixel_offset(x_offset, y_offset);
        pixel_center + sample_offset
    }

    fn calculate_viewport_extent(vertical_fov: f32, focal_length: f32, resolution: &Resolution) -> (f32, f32) {
        let fov_radians = f32::to_radians(vertical_fov);
        let height = f32::tan(fov_radians / 2.0);

        let viewport_height = 2.0 * height * focal_length;
        let viewport_width = viewport_height * resolution.aspect_ratio();

        (viewport_width, viewport_height)
    }
}
