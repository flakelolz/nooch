use crate::prelude::*;

#[allow(dead_code)]
#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u32)]
pub enum Buttons {
    None = 0,
    /// Neutral
    N = 1 << 0,
    /// Facing Directions
    FacingLeft = 1 << 1,
    FacingOpponent = 1 << 2,
    /// Down
    D = 1 << 3,
    /// Up
    U = 1 << 4,
    /// Right
    R = 1 << 5,
    /// Left
    L = 1 << 6,
    /// Up-left
    UL = (Self::U as u32 | Self::L as u32),
    /// Up-right
    UR = (Self::U as u32 | Self::R as u32),
    /// Down-left
    DL = (Self::D as u32 | Self::L as u32),
    /// Down-right
    DR = (Self::D as u32 | Self::R as u32),
    /// Light punch
    Lp = 1 << 7,
    /// Medium punch
    Mp = 1 << 8,
    /// Heavy punch
    Hp = 1 << 9,
    /// Light kick
    Lk = 1 << 10,
    /// Medium kick
    Mk = 1 << 11,
    /// Heavy kick
    Hk = 1 << 12,
    /// Any punch
    Punches = (Self::Lp as u32 | Self::Mp as u32 | Self::Hp as u32),
    /// Any kick
    Kicks = (Self::Lk as u32 | Self::Mk as u32 | Self::Hk as u32),
    /// Any attack
    Attacks = (Self::Lp as u32
        | Self::Mp as u32
        | Self::Hp as u32
        | Self::Lk as u32
        | Self::Mk as u32
        | Self::Hk as u32),
    /// Any direction
    Direction = (Self::D as u32 | Self::U as u32 | Self::R as u32 | Self::L as u32),
    /// Any attack or direction
    Any = (Self::Attacks as u32 | Self::Direction as u32),
}

impl Buttons {
    pub fn num(&self) -> u32 {
        *self as u32
    }

    pub fn pressed(&self, input: &Input) -> bool {
        input.pressed(*self)
    }
}

impl std::ops::BitOr for Buttons {
    type Output = u32;
    fn bitor(self, other: Self) -> u32 {
        self.num() | other.num()
    }
}

impl std::ops::BitAnd for Buttons {
    type Output = u32;
    fn bitand(self, other: Self) -> u32 {
        self.num() & other.num()
    }
}

impl std::ops::BitAnd<Buttons> for u32 {
    type Output = u32;
    fn bitand(self, other: Buttons) -> u32 {
        self & other.num()
    }
}

impl std::ops::BitOr<Buttons> for u32 {
    type Output = u32;
    fn bitor(self, other: Buttons) -> u32 {
        self | other.num()
    }
}

impl PartialEq<Buttons> for u32 {
    fn eq(&self, other: &Buttons) -> bool {
        *self == other.num()
    }
}

impl std::fmt::Binary for Buttons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.num())
    }
}

impl From<Input> for Buttons {
    fn from(num: Input) -> Self {
        if num.0 & Buttons::U == Buttons::U {
            return Buttons::U;
        }
        if num.0 & Buttons::D == Buttons::D {
            return Buttons::D;
        }
        if num.0 & Buttons::R == Buttons::R {
            return Buttons::R;
        }
        if num.0 & Buttons::L == Buttons::L {
            return Buttons::L;
        }
        if num.0 & Buttons::Lp == Buttons::Lp {
            return Buttons::Lp;
        }
        if num.0 & Buttons::Mp == Buttons::Mp {
            return Buttons::Mp;
        }
        if num.0 & Buttons::Hp == Buttons::Hp {
            return Buttons::Hp;
        }
        if num.0 & Buttons::Lk == Buttons::Lk {
            return Buttons::Lk;
        }
        if num.0 & Buttons::Mk == Buttons::Mk {
            return Buttons::Mk;
        }
        if num.0 & Buttons::Hk == Buttons::Hk {
            return Buttons::Hk;
        }
        Buttons::None
    }
}
