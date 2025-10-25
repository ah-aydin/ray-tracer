#[derive(Debug, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(a: f64, b: f64) -> Interval {
        let min = a.min(b);
        let max = a.max(b);
        Interval { min, max }
    }

    pub fn from_intervals(i1: &Interval, i2: &Interval) -> Interval {
        let min = if i1.min < i2.min { i1.min } else { i2.min };
        let max = if i1.max > i2.max { i1.max } else { i2.max };
        Interval { min, max }
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval {
            min: self.min + padding,
            max: self.max - padding,
        }
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.max(self.min).min(self.max)
    }
}
