use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Serialize, Deserialize, Eq, Clone, Copy)]
pub struct Square<const S: u8> {
    pub raw: u8,
}

impl<const S: u8> Square<S> {
    pub fn all() -> impl Iterator<Item = Self> {
        (0..S * S).map(|i| Self { raw: i })
    }

    #[must_use]
    pub fn new(x: u8, y: u8) -> Self {
        if x.max(y) >= S {
            Self { raw: u8::MAX }
        } else {
            Self {
                raw: x.saturating_add(y.saturating_mul(S)),
            }
        }
    }

    #[must_use]
    pub const fn out_of_bounds(&self) -> bool {
        self.raw > S * S
    }

    #[must_use]
    pub const fn x(&self) -> u8 {
        self.raw % S
    }

    #[must_use]
    pub const fn y(&self) -> u8 {
        self.raw / S
    }

    #[must_use]
    pub const fn adjacent(&self) -> [Self; 4] {
        let north = self.raw.wrapping_sub(S);
        let east = self.raw.saturating_add(1);
        let south = self.raw.saturating_add(S);
        let west = self.raw.wrapping_sub(1);
        [
            Self { raw: north },
            Self { raw: east },
            Self { raw: south },
            Self { raw: west },
        ]
    }

    #[must_use]
    pub const fn is_border(&self) -> bool {
        let x = self.x();
        let y = self.y();
        let s = S - 1;

        x % s == 0 || y % s == 0
    }
}

static CHARS: [char; 7] = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];
impl<const S: u8> fmt::Debug for Square<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = self.x();
        let y = self.y();
        let y = CHARS.get(y as usize).unwrap_or(&'_');
        if self.x() < 7 {
            write!(f, "{x}{y}")
        } else {
            write!(f, "_{y}")
        }
    }
}

impl<const S: u8> PartialEq for Square<S> {
    fn eq(&self, rhs: &Self) -> bool {
        self.raw == rhs.raw
    }
}

impl<const S: u8> std::hash::Hash for Square<S> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        u32::from(self.raw).hash(state);
    }
}
