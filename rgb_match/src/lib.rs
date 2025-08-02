#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match (first, second) {
            (a, b) if a == self.a && b == self.g => {
                self.a = b;
                self.g = a;
            }
            (a, b) if a == self.a && b == self.b => {
                self.a = b;
                self.b = a;
            }
            (a, b) if a == self.a && b == self.r => {
                self.a = b;
                self.r = a;
            }
            (a, b) if a == self.g && b == self.a => {
                self.g = b;
                self.a = a;
            }
            (a, b) if a == self.g && b == self.b => {
                self.g = b;
                self.b = a;
            }
            (a, b) if a == self.g && b == self.r => {
                self.g = b;
                self.r = a;
            }
            (a, b) if a == self.b && b == self.a => {
                self.b = b;
                self.a = a;
            }
            (a, b) if a == self.b && b == self.g => {
                self.b = b;
                self.g = a;
            }
            (a, b) if a == self.b && b == self.r => {
                self.b = b;
                self.r = a;
            }
            (a, b) if a == self.r && b == self.a => {
                self.r = b;
                self.a = a;
            }
            (a, b) if a == self.r && b == self.g => {
                self.r = b;
                self.g = a;
            }
            (a, b) if a == self.r && b == self.b => {
                self.r = b;
                self.b = a;
            }
            _ => {}
        }
        self
    }
}
