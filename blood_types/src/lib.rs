#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    rh_factor: RhFactor,
}

use std::{ str::FromStr, fmt::{ self, Debug }, cmp::{ Ord, Ordering } };

impl FromStr for Antigen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "AB" => Ok(Self::AB),
            "B" => Ok(Self::B),
            "O" => Ok(Self::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Positive),
            "-" => Ok(Self::Negative),
            _ => Err(()),
        }
    }
}

impl FromStr for BloodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_str, rh_str) = s.split_at(s.len() - 1);
        Ok(BloodType {
            antigen: antigen_str.parse()?,
            rh_factor: rh_str.parse()?,
        })
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.antigen, &self.rh_factor).cmp(&(&other.antigen, &other.rh_factor))
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{:?}{}", self.antigen, rh)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.antigen == Antigen::AB {
            return true;
        }
        let antigen_ok = matches!(
            (self.antigen, other.antigen),
            (Antigen::A, Antigen::A | Antigen::O) |
                (Antigen::B, Antigen::B | Antigen::O) |
                (Antigen::O, Antigen::O)
        );

        let rh_ok = self.rh_factor == RhFactor::Positive || other.rh_factor == RhFactor::Negative;

        antigen_ok && rh_ok
    }

    pub fn donors(&self) -> Vec<Self> {
        all_blood_types()
            .into_iter()
            .filter(|bt| self.can_receive_from(bt))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        all_blood_types()
            .into_iter()
            .filter(|bt| bt.can_receive_from(self))
            .collect()
    }
}

pub fn all_blood_types() -> Vec<BloodType> {
    vec![
        BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
        BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
        BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
        BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
        BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
        BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
        BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
        BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative }
    ]
}
