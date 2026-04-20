// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {
    pub fn new(value: u16) -> Self {
        Self { value }
    }
}

// impl From<&SaturatingU16> for u16 {
//     fn from(value: &SaturatingU16) -> Self {
//         value.value
//     }
// }

// impl From<SaturatingU16> for u16 {
//     fn from(value: SaturatingU16) -> Self {
//         value.value
//     }
// }

// impl From<SaturatingU16> for u8 {
//     fn from(value: SaturatingU16) -> Self {
//         <u16 as TryInto<u8>>::try_into(value.value).expect("unable to reduce")
//     }
// }

// impl From<&SaturatingU16> for u8 {
//     fn from(value: &SaturatingU16) -> Self {
//         <u16 as TryInto<u8>>::try_into(value.value).expect("unable to reduce")
//     }
// }

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        let value = value.into();
        Self { value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        let value = Into::<u16>::into(*value);
        Self::new(value)
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self { value: *value }
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
            SaturatingU16::new(self.value.saturating_add(rhs.value))
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16 { value: self.value + *rhs }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16::new(self.value.saturating_add(rhs))
    }
}


// impl Add<u8> for SaturatingU16 {
//     type Output = SaturatingU16;

//     fn add(self, rhs: u8) -> Self::Output {
//         SaturatingU16 {
//             value: self.value + Into::<u16>::into(rhs),
//         }
//     }
// }

// impl Add<&u8> for SaturatingU16 {
//     type Output = SaturatingU16;

//     fn add(self, rhs: &u8) -> Self::Output {
//         SaturatingU16 {
//             value: self.value + Into::<u16>::into(*rhs),
//         }
//     }
// }


// impl Add<&SaturatingU16> for u16 {
//     type Output = SaturatingU16;

//     fn add(self, rhs: &SaturatingU16) -> Self::Output {
//         SaturatingU16 {
//             value: self + rhs.value,
//         }
//     }
// }

// impl Add<&SaturatingU16> for u8 {
//     type Output = SaturatingU16;

//     fn add(self, rhs: &SaturatingU16) -> Self::Output {
//         SaturatingU16 {
//             value: u16::from(self) + rhs.value,
//         }
//     }
// }

// impl Add<SaturatingU16> for u8 {
//     type Output = SaturatingU16;

//     fn add(self, rhs: SaturatingU16) -> Self::Output {
//         SaturatingU16 {
//             value: u16::from(self) + rhs.value,
//         }
//     }
// }

// impl Add<SaturatingU16> for &u8 {
//     type Output = SaturatingU16;

//     fn add(self, rhs: SaturatingU16) -> Self::Output {
//         SaturatingU16 {
//             value: u16::from(*self) + rhs.value,
//         }
//     }
// }

// impl Add<&SaturatingU16> for &u8 {
//     type Output = SaturatingU16;

//     fn add(self, rhs: &SaturatingU16) -> Self::Output {
//         SaturatingU16 {
//             value: u16::from(*self) + rhs.value,
//         }
//     }
// }
