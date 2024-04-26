
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

    pub fn fully_contains(&self, x: f64) -> bool {
        return self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { return self.min; }
        if x > self.max { return self.max; }
        x
    }

    pub fn len(&self) -> f64 {
        self.max - self.min
    }

    pub fn is_empty(&self) -> bool {
        self.min >= self.max
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;

        Interval::new(self.min - padding, self.max + padding)
    }

    pub fn intersect(&self, other: Interval) -> Interval {
        let intersection_min = self.min.max(other.min);
        let intersection_max = self.max.min(other.max);

        Interval::new(intersection_min, intersection_max)
    }

    pub fn unite(&self, other: Interval) -> Interval {
        let union_min = self.min.min(other.min);
        let union_max = self.max.max(other.max);

        Interval::new(union_min, union_max)
    }
}

impl std::ops::Add<f64> for Interval {
    type Output = Interval;

    fn add(self, rhs: f64) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}

impl std::ops::Add<Interval> for f64 {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Self::Output {
        Interval::new(self + rhs.min, self + rhs.max)
    }
}

pub const EMPTY: Interval = Interval::new_const(f64::INFINITY, f64::NEG_INFINITY);
pub const UNIVERSE: Interval = Interval::new_const(f64::NEG_INFINITY, f64::INFINITY);
