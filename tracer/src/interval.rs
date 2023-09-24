

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Default for Interval {
    fn default() -> Self {
        // default interval is empty
        Interval { min: f64::INFINITY, max: f64::NEG_INFINITY }
    }
}

impl Interval {
    pub fn new(_min: f64, _max: f64) -> Interval {
        Interval { min: _min, max: _max }
    }
    
    pub const fn new_const(_min: f64, _max: f64) -> Interval {
        Interval { min: _min, max: _max }
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { return self.min; }
        if x > self.max { return self.max; }
        x
    }
}

pub const EMPTY: Interval = Interval::new_const(f64::INFINITY, f64::NEG_INFINITY);
pub const UNIVERSE: Interval = Interval::new_const(f64::NEG_INFINITY, f64::INFINITY);
