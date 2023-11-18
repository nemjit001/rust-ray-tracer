pub struct Interval(f32, f32);

impl Default for Interval {
    fn default() -> Self {
        Interval::empty()
    }
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Interval(min, max)
    }

    pub fn empty() -> Self {
        Interval::new(f32::INFINITY, f32::NEG_INFINITY)
    }

    pub fn univers() -> Self {
        Interval::new(f32::NEG_INFINITY, f32::INFINITY)
    }

    pub fn min(&self) -> f32 {
        self.0
    }

    pub fn max(&self) -> f32 {
        self.1
    }

    pub fn contains(&self, value: f32) -> bool {
        self.min() <= value && value <= self.max()
    }

    pub fn surrounds(&self, value: f32) -> bool {
        self.min() < value && value < self.max()
    }
}
