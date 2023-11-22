#[derive(Debug, Clone, Copy)]
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

    pub fn universe() -> Self {
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

    pub fn clamp(&self, value: f32) -> f32 {
        if value < self.min() {
            return self.min()
        }
        else if value > self.max() {
            return self.max()
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let interval = Interval::empty();
        assert!(!interval.contains(0.0))
    }

    #[test]
    fn test_universe() {
        let interval = Interval::universe();
        assert!(interval.contains(0.0))
    }

    #[test]
    fn test_contains() {
        let interval = Interval::new(0.0, 10.0);
        assert!(interval.contains(5.0));
        assert!(interval.contains(0.0));
        assert!(interval.contains(10.0));
        assert!(!interval.contains(-42.0));
    }

    #[test]
    fn test_surrounds() {
        let interval = Interval::new(0.0, 10.0);
        assert!(interval.surrounds(5.0));
        assert!(!interval.surrounds(0.0));
        assert!(!interval.surrounds(10.0));
        assert!(!interval.surrounds(-42.0));
    }
}
