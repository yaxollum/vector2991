#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        self.x.hypot(self.y)
    }
    pub fn abs_bearing(&self) -> f64 {
        (if self.x < 0.0 && self.y > 0.0 {
            450.0
        } else {
            90.0
        }) - self.y.atan2(self.x).to_degrees()
    }
    pub fn from_dir(ang: f64) -> Self {
        let ang = ang.to_radians();
        Self {
            x: ang.cos(),
            y: ang.sin(),
        }
    }
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn scale(&self, n: f64) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
