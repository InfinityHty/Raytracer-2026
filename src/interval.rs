pub struct Interval {
    pub min: f64,
    pub max: f64,
}
impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn expand(&self, delta: f64) -> Interval {
        Interval {
            min: self.min - delta / 2.0,
            max: self.max + delta / 2.0,
        }
    }
    pub fn contains(&self, t: f64) -> bool {
        self.min <= t && t <= self.max
    }
    pub fn surround(&self, t: f64) -> bool {
        self.min < t && t < self.max
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
}
