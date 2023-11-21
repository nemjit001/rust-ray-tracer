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

    pub fn dimensions(&self) -> (u32, u32) {
        (self.0, self.1)
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width() as f32 / self.height() as f32
    }
}
