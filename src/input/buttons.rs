use crate::prelude::*;

#[allow(dead_code)]
#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Buttons {
    None = 0,
    // Direcitons
    Down = 1 << 0,
    Up = 1 << 1,
    Right = 1 << 2,
    Left = 1 << 3,
    // Diagonals
    UpLeft = (Self::Up as isize | Self::Left as isize),
    UpRight = (Self::Up as isize | Self::Right as isize),
    DownLeft = (Self::Down as isize | Self::Left as isize),
    DownRight = (Self::Down as isize | Self::Right as isize),
    // Attacks
    Lp = 1 << 4,
    Mp = 1 << 5,
    Hp = 1 << 6,
    Lk = 1 << 7,
    Mk = 1 << 8,
    Hk = 1 << 9,
    // Combinations
    AnyAttack = (Self::Lp as isize
        | Self::Mp as isize
        | Self::Hp as isize
        | Self::Lk as isize
        | Self::Mk as isize
        | Self::Hk as isize),
    AnyDirection =
        (Self::Down as isize | Self::Up as isize | Self::Right as isize | Self::Left as isize),
    AnyAttackOrDirection = (Self::AnyAttack as isize | Self::AnyDirection as isize),
    // Released buttons
    ReleaseBitShiftModifier = 20,
    ReleaseDown = ((Self::Down as isize) << Self::ReleaseBitShiftModifier as isize),
    ReleaseUp = ((Self::Up as isize) << Self::ReleaseBitShiftModifier as isize),
    ReleaseRight = ((Self::Right as isize) << Self::ReleaseBitShiftModifier as isize),
    ReleaseLeft = ((Self::Left as isize) << Self::ReleaseBitShiftModifier as isize),
    ReleaseLp = ((Self::Lp as isize) << Self::ReleaseBitShiftModifier as isize),
    ReleaseMp = ((Self::Mp as isize) << Self::ReleaseBitShiftModifier as isize),
    ReleaseHp = ((Self::Hp as isize) << Self::ReleaseBitShiftModifier as isize),
    ReleaseLk = ((Self::Lk as isize) << Self::ReleaseBitShiftModifier as isize),
    ReleaseMk = ((Self::Mk as isize) << Self::ReleaseBitShiftModifier as isize),
    ReleaseHk = ((Self::Hk as isize) << Self::ReleaseBitShiftModifier as isize),
}

impl Buttons {
    pub fn bits(&self) -> u32 {
        *self as u32
    }
}

impl std::ops::BitOr for Buttons {
    type Output = u32;
    fn bitor(self, other: Self) -> u32 {
        self.bits() | other.bits()
    }
}

impl std::ops::BitAnd for Buttons {
    type Output = u32;
    fn bitand(self, other: Self) -> u32 {
        self.bits() & other.bits()
    }
}

impl std::ops::BitAnd<Buttons> for u32 {
    type Output = u32;
    fn bitand(self, other: Buttons) -> u32 {
        self & other.bits()
    }
}

impl std::ops::BitOr<Buttons> for u32 {
    type Output = u32;
    fn bitor(self, other: Buttons) -> u32 {
        self | other.bits()
    }
}

impl PartialEq<Buttons> for u32 {
    fn eq(&self, other: &Buttons) -> bool {
        *self == other.bits()
    }
}

impl std::fmt::Binary for Buttons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.bits())
    }
}
