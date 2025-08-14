#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::str::FromStr;

impl RhFactor {
    pub fn to_symbol(&self) -> String {
        match self {
            RhFactor::Positive => "+".to_string(),
            RhFactor::Negative => "-".to_string(),
        }
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rh_factor = if s.ends_with('+') {
            RhFactor::Positive
        } else if s.ends_with('-') {
            RhFactor::Negative
        } else {
            return Err("Invalid Rh factor".to_string());
        };

        let antigen_str = &s[..s.len() - 1];
        let antigen = match antigen_str {
            "A" => Antigen::A,
            "B" => Antigen::B,
            "AB" => Antigen::AB,
            "O" => Antigen::O,
            _ => {
                return Err("Invalid antigen".to_string());
            }
        };

        Ok(BloodType { antigen, rh_factor })
    }
}
use std::fmt::Debug;
use core::fmt;
impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{}", self.antigen, self.rh_factor.to_symbol())
    }
}
impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let rh_compatible = match (self.rh_factor, other.rh_factor) {
            (RhFactor::Positive, _) => true,
            (RhFactor::Negative, RhFactor::Negative) => true,
            (RhFactor::Negative, RhFactor::Positive) => false,
        };

        if !rh_compatible {
            return false;
        }

        match (self.antigen, other.antigen) {
            (Antigen::AB, _) => true,
            (Antigen::A, Antigen::A) | (Antigen::A, Antigen::O) => true,
            (Antigen::B, Antigen::B) | (Antigen::B, Antigen::O) => true,
            (Antigen::O, Antigen::O) => true,
            _ => false,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut donors = Vec::new();

        let antigens = [Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        let rh_factors = [RhFactor::Positive, RhFactor::Negative];

        for antigen in antigens.iter() {
            for rh_factor in rh_factors.iter() {
                let donor = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                };

                if self.can_receive_from(&donor) {
                    donors.push(donor);
                }
            }
        }

        donors
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = Vec::new();

        let antigens = [Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        let rh_factors = [RhFactor::Positive, RhFactor::Negative];

        for antigen in antigens.iter() {
            for rh_factor in rh_factors.iter() {
                let recipient = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                };

                if recipient.can_receive_from(self) {
                    recipients.push(recipient);
                }
            }
        }

        recipients
    }
}
