#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}
pub const PI: f64 = 3.14159265358979323846264338327950288f64;

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self { center: Point(x, y), radius: radius }
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
    pub fn intersect(&self, other: Circle) -> bool {
        let distance = self.center.distance(other.center);
        distance < self.radius + other.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}
