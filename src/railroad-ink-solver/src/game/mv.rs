use super::Roll;
use crate::board::placement::Placement;
use serde_with::serde_as;
use serde_with::SerializeDisplay;
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[serde_as]
#[derive(Debug, Clone, Copy, Eq, SerializeDisplay)]
pub enum Move {
    #[serde_as(as = "DisplayFromStr")]
    Place(Placement),
    #[serde_as(as = "DisplayFromStr")]
    SetRoll(Roll),
    Roll,
    End,
}

use Move::{Place, SetRoll};

impl PartialEq for Move {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Place(a), Place(b)) => a == b,
            (SetRoll(a), SetRoll(b)) => a == b,
            (Move::Roll, Move::Roll) | (Move::End, Move::End) => true,
            _ => false,
        }
    }
}

impl Hash for Move {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let base = match self {
            Move::End => 0,
            Move::Roll => 1,
            Move::Place(placement) => (u32::from(placement) << 2) + 2,
            Move::SetRoll(roll) => (u32::from(roll) << 2) + 3,
        };
        base.hash(state);
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for Move {
    type Err = ();
    fn from_str(input: &str) -> Result<Move, Self::Err> {
        match input {
            "Roll" => Ok(Move::Roll),
            "End" => Ok(Move::End),
            _ if input.starts_with("SetRoll") => {
                let payload = String::from(input)[9..input.len() - 2].to_string();
                let integers: Vec<u8> = payload
                    .split(',')
                    .map(|string| {
                        string.parse().unwrap_or_else(|_| {
                            panic!(
                                "Move::SetRoll parse failed. Could not parse integer: {}",
                                string
                            )
                        })
                    })
                    .collect();
                let integers = integers.into_boxed_slice();
                let integers: Box<[u8; 4]> = integers.try_into().unwrap_or_else(|_| {
                    panic!(
                        "Move::SetRoll parse failed. Expected 4 integers. Got {}",
                        payload
                    )
                });

                Ok(Move::SetRoll(Roll(*integers)))
            }
            _ if input.starts_with("Place") => {
                let payload = String::from(input)[6..input.len() - 1].to_string();
                let placement = Placement::from_str(payload.as_str()).unwrap_or_else(|_| {
                    panic!("Move::Place Placement parse failed. Got {}", payload)
                });
                Ok(Move::Place(placement))
            }
            _ => Err(()),
        }
    }
}
