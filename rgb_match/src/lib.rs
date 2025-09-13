#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(self, a: u8, b: u8) -> Self {
        if a == b {
            return self;
        }

        #[inline]
        fn s(v: u8, a: u8, b: u8) -> u8 {
            if v == a {
                b
            } else if v == b {
                a
            } else {
                v
            }
        }

        Self {
            r: s(self.r, a, b),
            g: s(self.g, a, b),
            b: s(self.b, a, b),
            a: s(self.a, a, b),
        }
    }
}