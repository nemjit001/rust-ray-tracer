use nalgebra_glm::Vec3;
use rand::{thread_rng, Rng};

use crate::resolution::Resolution;
use crate::interval::Interval;

// const WORLD_FORWARD: Vec3   = Vec3::new(0.0, 0.0, 1.0);
// const WORLD_RIGHT: Vec3     = Vec3::new(1.0, 0.0, 0.0);
const WORLD_UP: Vec3        = Vec3::new(0.0, 1.0, 0.0);

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
        let vec_v = -1.0 * viewport_height * camera_vectors.right();

        let pixel_delta_u = vec_u / resolution.width() as f32;
        let pixel_delta_v = vec_v / resolution.height() as f32;
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

struct DefocusDisk {
    angle: f32,
    radius: f32,
}

impl DefocusDisk {
    pub fn new(angle: f32, focal_length: f32) -> Self {
        let radius = focal_length * f32::tan(f32::to_radians(angle / 2.0));

        DefocusDisk {
            angle,
            radius,
        }
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn sample(&self, camera_vectors: &CameraVectors) -> Vec3 {
        let disk_u = self.radius * camera_vectors.right();
        let disk_v = -1.0 * self.radius * camera_vectors.up();

        let random_sample = self.random_in_unit_disk();
        (random_sample.x * disk_u) + (random_sample.y * disk_v)
    }

    fn random_in_unit_disk(&self) -> Vec3 {
        let mut rng = thread_rng();

        loop {
            let vec = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                0.0
            );

            if vec.magnitude_squared() < 1.0 {
                return vec
            }
        }
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

pub enum FocusMode {
    AutoFocus,
    Manual(f32),
}

pub struct Camera {
    position: Vec3,
    scene_depth: Interval,
    view_plane: ViewPlane,
    defocus_disk: DefocusDisk,
    camera_vectors: CameraVectors,
}

impl Camera {
    pub fn new(position: Vec3, look_at: Vec3, vertical_fov: f32, focus_mode: FocusMode, defocus_angle: f32, scene_depth: Interval, resolution: &Resolution) -> Self {
        let focal_length = match focus_mode {
            FocusMode::AutoFocus => (look_at - position).magnitude(),
            FocusMode::Manual(length) => length,
        };

        let (viewport_width, viewport_height) = Self::calculate_viewport_extent(vertical_fov, focal_length, resolution);

        let forward = (position - look_at).normalize();
        let up = WORLD_UP.cross(&forward);
        let camera_vectors = CameraVectors {
            forward,
            up,
        };
        
        let view_plane = ViewPlane::new(&position, focal_length, &camera_vectors, viewport_width, viewport_height, resolution);
        let defocus_disk = DefocusDisk::new(defocus_angle, focal_length);

        Camera {
            position,
            scene_depth,
            view_plane,
            defocus_disk,
            camera_vectors,
        }
    }

    pub fn scene_depth_interval(&self) -> &Interval {
        &self.scene_depth
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

    pub fn get_ray_origin(&self) -> Vec3 {
        if self.defocus_disk.angle() <= 0.0 {
            self.position
        }
        else {
            self.position + self.defocus_disk.sample(&self.camera_vectors)
        }
    }

    fn calculate_viewport_extent(vertical_fov: f32, focal_length: f32, resolution: &Resolution) -> (f32, f32) {
        let fov_radians = f32::to_radians(vertical_fov);
        let height = f32::tan(fov_radians / 2.0);

        let viewport_height = 2.0 * height * focal_length;
        let viewport_width = viewport_height * resolution.aspect_ratio();

        (viewport_width, viewport_height)
    }
}
