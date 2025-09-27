use crate::RomanDigit::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl RomanDigit {
    const fn value(self) -> u32 {
        match self {
            I => 1,
            V => 5,
            X => 10,
            L => 50,
            C => 100,
            D => 500,
            M => 1000,
            Nulla => 0,
        }
    }
}

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut digits = Vec::new();
        const VALUES: [(u32, &[RomanDigit]); 13] = [
            (1000, &[M]),
            (900, &[C, M]),
            (500, &[D]),
            (400, &[C, D]),
            (100, &[C]),
            (90, &[X, C]),
            (50, &[L]),
            (40, &[X, L]),
            (10, &[X]),
            (9, &[I, X]),
            (5, &[V]),
            (4, &[I, V]),
            (1, &[I]),
        ];

        for (value, roman_digits) in VALUES.iter() {
            while num >= *value {
                digits.extend_from_slice(roman_digits);
                num -= value;
            }
        }

        RomanNumber(digits)
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_number()
    }
}

impl RomanNumber {
    pub fn next_number(&self) -> Option<RomanNumber> {
        let current: u32 = self.clone().into();
        current.checked_add(1).map(RomanNumber::from)
    }
}

impl From<RomanNumber> for u32 {
    fn from(roman: RomanNumber) -> Self {
        let mut total = 0i32;
        let digits = &roman.0;

        for i in 0..digits.len() {
            let current_value = digits[i].value() as i32;
            let next_value = digits.get(i + 1).map_or(0, |d| d.value() as i32);

            if current_value < next_value {
                total -= current_value;
            } else {
                total += current_value;
            }
        }

        total as u32
    }
}
