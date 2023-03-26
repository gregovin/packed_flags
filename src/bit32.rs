use std::{ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, Shl, ShlAssign,
    Shr, ShrAssign, Sub,
}, fmt::{UpperHex, LowerHex, Octal, Binary}};

use crate::{flag_iter, FlagLs, B64, B128, Bsize, Blong, FlagLsError};

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A list of flags up to 32 flags long, or a 32 bit bitfield
pub struct B32 {
    inner: u32,
    len: usize,
}
impl B32 {
    fn lower_mask(point: usize) -> u32 {
        if point > 32 {
            panic!("Cannot create mask more than 32 bits for B32")
        } else {
            (1 << point) - 1
        }
    }
    const fn inner(&self) -> u32 {
        self.inner
    }
    #[must_use]
    /// Converts the bitfield into its inner representation, a u32, consuming it
    pub const fn as_inner(self) -> u32 {
        self.inner
    }
    fn uper_mask(point: usize) -> u32 {
        if point > 32 {
            panic!("Cannot mask above the end of the list");
        } else {
            u32::MAX - (1 << point) + 1
        }
    }
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    const fn init(inner: u32, len: usize) -> Self {
        Self { inner, len }
    }
}
impl Index<usize> for B32 {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index<self.len,"Cannot get element {} of flag list of length {}",index,self.len);
        if (self.inner >> index) & 1 == 1 {
            &true
        } else {
            &false
        }
    }
}
impl FlagLs for B32 {
    const MAX_LENGTH: usize = 32;

    fn len(&self) -> usize {
        self.len
    }

    fn set_len(&mut self, new_len: usize) {
        assert!(new_len<=Self::MAX_LENGTH,"Cannot set length to a length larger than 32 for B32");
        self.len = new_len;
        self.inner &= (1 << new_len) - 1;
    }

    fn insert(&mut self, index: usize, flag: bool) {
        if index > self.len {
            panic!("Cannot insert out of bounds");
        } else if self.len == Self::MAX_LENGTH {
            panic!("cannot insert to full list of flags")
        } else {
            let uper = self.inner & Self::uper_mask(index);
            let lower = self.inner & Self::lower_mask(index);
            self.inner = (uper << 1) + (u32::from(flag) << index) + lower;
            self.len += 1;
        }
    }

    fn remove(&mut self, index: usize) -> bool {
        if index >= self.len {
            panic!("Cannot remove out of bounds");
        } else {
            let uper = self.inner & Self::uper_mask(index + 1);
            let lower = self.inner & Self::lower_mask(index);
            let out = (self.inner >> index) & 1;
            self.inner = (uper >> 1) + lower;
            self.len -= 1;
            out == 1
        }
    }

    fn clear(&mut self) {
        self.inner = 0;
        self.len = 0;
    }

    fn get(&self, index: usize) -> Option<bool> {
        if index<self.len{
            Some((self.inner >> index) & 1 == 1)   
        } else {
            None
        }
    }

    fn set(&mut self, index: usize, flag: bool) {
        if index < self.len() {
            let uper = self.inner & Self::uper_mask(index + 1);
            let lower = self.inner & Self::lower_mask(index);
            self.inner = uper + (u32::from(flag) << index) + lower;
        } else {
            panic!("Cannot set out of bounds")
        }
    }

    fn iter(&self) -> flag_iter::Iter<Self> {
        flag_iter::Iter::new(self)
    }
}
impl BitAnd<Self> for B32 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::init(self.inner() & rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitAndAssign<Self> for B32 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner &= rhs.inner();
        self.len = self.len.max(rhs.len());
    }
}
impl BitOr<Self> for B32 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::init(self.inner() | rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitOrAssign<Self> for B32 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner |= rhs.inner();
        self.len = self.len.max(rhs.len());
    }
}
impl BitXor<Self> for B32 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::init(self.inner().bitxor(rhs.inner()), self.len().max(rhs.len()))
    }
}
impl BitXorAssign<Self> for B32 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner());
        self.len = self.len.max(rhs.len());
    }
}
impl Shl<usize> for B32 {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        Self::init(self.inner << rhs, (self.len + rhs).min(Self::MAX_LENGTH))
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl ShlAssign<usize> for B32 {
    fn shl_assign(&mut self, rhs: usize) {
        self.inner.shl_assign(rhs);
        self.len = (self.len + rhs).min(Self::MAX_LENGTH);
    }
}
impl Shr<usize> for B32 {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output {
        let new_len = if self.len > rhs { self.len - rhs } else { 0 };
        Self::init(self.inner >> rhs, new_len)
    }
}
impl ShrAssign<usize> for B32 {
    fn shr_assign(&mut self, rhs: usize) {
        self.inner.shr_assign(rhs);
        self.len = if self.len > rhs { self.len - rhs } else { 0 };
    }
}
impl Not for B32 {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self::init((!self.inner) & Self::lower_mask(self.len), self.len)
    }
}
impl Sub<Self> for B32 {
    type Output = Self;
    ///note subtraction is set difference
    fn sub(self, rhs: Self) -> Self::Output {
        Self::init(self.inner & (!rhs.inner), self.len)
    }
}
impl TryFrom<B64> for B32{
    type Error = FlagLsError;
    fn try_from(value: B64) -> Result<Self, Self::Error> {
        let len=value.len();
        if len<Self::MAX_LENGTH{
            Err(FlagLsError::MaximumLengthExceeded { mx_len: Self::MAX_LENGTH, attempt_len: len })
        } else {
            Ok(Self { inner: value.as_inner().try_into().expect("Infalible"), len})
        }
    }
}
impl TryFrom<B128> for B32 {
    type Error = FlagLsError;
    fn try_from(value: B128) -> Result<Self, Self::Error> {
        let len=value.len();
        if len<Self::MAX_LENGTH{
            Err(FlagLsError::MaximumLengthExceeded { mx_len: Self::MAX_LENGTH, attempt_len: len })
        } else {
            Ok(Self { inner: value.as_inner().try_into().expect("Infalible"), len})
        }
    }
}
impl TryFrom<Bsize> for B32{
    type Error = FlagLsError;
    fn try_from(value: Bsize) -> Result<Self, Self::Error> {
        let len=value.len();
        if len<Self::MAX_LENGTH{
            Err(FlagLsError::MaximumLengthExceeded { mx_len: Self::MAX_LENGTH, attempt_len: len })
        } else {
            Ok(Self { inner: value.as_inner().try_into().expect("Infalible"), len})
        }
    }
}
impl TryFrom<Blong> for B32{
    type Error = FlagLsError;
    fn try_from(value: Blong) -> Result<Self, Self::Error> {
        let len=value.len();
        if len<Self::MAX_LENGTH{
            Err(FlagLsError::MaximumLengthExceeded { mx_len: Self::MAX_LENGTH, attempt_len: len })
        } else {
            Ok(Self { inner: (*value.as_inner().first().unwrap_or(&0)).try_into().expect("Infalible"), len})
        }
    }
}
impl UpperHex for B32{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:X}",self.inner)
    }
}
impl LowerHex for B32{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:x}",self.inner)
    }
}
impl Octal for B32{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:o}",self.inner)
    }
}
impl Binary for B32{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:b}",self.inner)
    }
}