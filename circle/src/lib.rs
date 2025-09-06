#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, rad: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius: rad,
        }
    }
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn intersect(&self, other: Self) -> bool {
        self.center.distance(other.center) <= self.radius + other.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: Self) -> f64 {
        // let res =0.0;
        let x = (other.0 - self.0).powi(2);
        let y = (other.1 - self.1).powi(2);
        (x + y).sqrt()
    }
}
