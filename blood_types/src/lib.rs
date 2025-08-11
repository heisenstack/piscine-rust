use std::cmp::{Ord, Ordering};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

impl FromStr for Antigen {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            other => Err(format!("`{}` is not a valid antigen", other)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            other => Err(format!("`{}` is not a valid Rh Factor", other)),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rh_factor == other.rh_factor {
            self.antigen.cmp(&other.antigen)
        } else {
            self.antigen.cmp(&other.antigen)
        }
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let len = value.len();
        if len < 2 || len > 3 {
            return Err(format!("Invalid antigen: `{}` invalid length: {}", value, len));
        }

        let rh_str = value.get(len - 1..).ok_or_else(|| format!("Invalid suffix"))?;
        let antigen_str = value.get(..len - 1).unwrap();

        let rh_factor = rh_str.parse()?;
        let antigen = antigen_str.parse()?;

        Ok(BloodType { antigen, rh_factor })
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.antigen)?;
        match self.rh_factor {
            RhFactor::Positive => write!(f, "+"),
            RhFactor::Negative => write!(f, "-"),
        }
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.rh_factor != other.rh_factor && self.rh_factor == RhFactor::Negative {
            return false;
        }
        if other.antigen == Antigen::O {
            return true;
        }
        self.antigen == Antigen::AB || self.antigen == other.antigen
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut result = Vec::new();

        let mut possible_antigens = if self.antigen == Antigen::O {
            vec![Antigen::O]
        } else {
            vec![Antigen::O, self.antigen.clone()]
        };

        if self.antigen == Antigen::AB {
            possible_antigens.extend(vec![Antigen::A, Antigen::B]);
        }

        let possible_rh = if self.rh_factor == RhFactor::Negative {
            vec![RhFactor::Negative]
        } else {
            vec![RhFactor::Positive, RhFactor::Negative]
        };

        for rh in &possible_rh {
            for antigen in &possible_antigens {
                result.push(BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh.clone(),
                });
            }
        }

        result
    }

    pub fn recipients(&self) -> Vec<Self> {
        let mut result = Vec::new();

        let mut possible_antigens = if self.antigen != Antigen::AB {
            vec![Antigen::AB, self.antigen.clone()]
        } else {
            vec![Antigen::AB]
        };

        if self.antigen == Antigen::O {
            possible_antigens.extend(vec![Antigen::A, Antigen::B]);
        }

        let possible_rh = if self.rh_factor == RhFactor::Negative {
            vec![RhFactor::Positive, RhFactor::Negative]
        } else {
            vec![RhFactor::Positive]
        };

        for rh in &possible_rh {
            for antigen in &possible_antigens {
                result.push(BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh.clone(),
                });
            }
        }

        result
    }
}
