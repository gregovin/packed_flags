use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, Shl, ShlAssign,
    Shr, ShrAssign, Sub,
};

use crate::{flag_iter, FlagLs, B64, B128, Bsize, Blong};

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug, Hash)]
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
    fn inner(&self) -> u32 {
        self.inner
    }
    #[must_use]
    /// Converts the bitfield into its inner representation, a u32, consuming it
    pub fn as_inner(self) -> u32 {
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
    pub fn new() -> B32 {
        B32::default()
    }
    fn init(inner: u32, len: usize) -> B32 {
        B32 { inner, len }
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

    unsafe fn get_unchecked(&self, index: usize) -> bool {
        (self.inner >> index) & 1 == 1
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

    fn iter(&self) -> flag_iter::Iter<B32> {
        flag_iter::Iter::new(self)
    }
}
impl BitAnd<B32> for B32 {
    type Output = B32;

    fn bitand(self, rhs: B32) -> Self::Output {
        B32::init(self.inner() & rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitAndAssign<B32> for B32 {
    fn bitand_assign(&mut self, rhs: B32) {
        self.inner &= rhs.inner();
        self.len = self.len.max(rhs.len());
    }
}
impl BitOr<B32> for B32 {
    type Output = B32;
    fn bitor(self, rhs: B32) -> Self::Output {
        B32::init(self.inner() | rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitOrAssign<B32> for B32 {
    fn bitor_assign(&mut self, rhs: B32) {
        self.inner |= rhs.inner();
        self.len = self.len.max(rhs.len());
    }
}
impl BitXor<B32> for B32 {
    type Output = B32;
    fn bitxor(self, rhs: B32) -> Self::Output {
        B32::init(self.inner().bitxor(rhs.inner()), self.len().max(rhs.len()))
    }
}
impl BitXorAssign<B32> for B32 {
    fn bitxor_assign(&mut self, rhs: B32) {
        self.inner.bitxor_assign(rhs.inner());
        self.len = self.len.max(rhs.len());
    }
}
impl Shl<usize> for B32 {
    type Output = B32;
    fn shl(self, rhs: usize) -> Self::Output {
        B32::init(self.inner << rhs, (self.len + rhs).min(Self::MAX_LENGTH))
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
    type Output = B32;
    fn shr(self, rhs: usize) -> Self::Output {
        let new_len = if self.len > rhs { self.len - rhs } else { 0 };
        B32::init(self.inner >> rhs, new_len)
    }
}
impl ShrAssign<usize> for B32 {
    fn shr_assign(&mut self, rhs: usize) {
        self.inner.shr_assign(rhs);
        self.len = if self.len > rhs { self.len - rhs } else { 0 };
    }
}
impl Not for B32 {
    type Output = B32;
    fn not(self) -> Self::Output {
        B32::init((!self.inner) & Self::lower_mask(self.len), self.len)
    }
}
impl Sub<B32> for B32 {
    type Output = B32;
    ///note subtraction is set difference
    fn sub(self, rhs: B32) -> Self::Output {
        B32::init(self.inner & (!rhs.inner), self.len)
    }
}
impl TryFrom<B64> for B32{
    type Error = String;
    fn try_from(value: B64) -> Result<Self, Self::Error> {
        let len=value.len();
        if len<Self::MAX_LENGTH{
            Err(format!("B32 only accepts flag lists less than 32 bits long, input had {len} bits"))
        } else {
            Ok(Self { inner: value.as_inner().try_into().unwrap(), len})
        }
    }
}
impl TryFrom<B128> for B32 {
    type Error = String;
    fn try_from(value: B128) -> Result<Self, Self::Error> {
        let len=value.len();
        if len<Self::MAX_LENGTH{
            Err(format!("B32 only accepts flag lists less than 32 bits long, input had {len} bits"))
        } else {
            Ok(Self { inner: value.as_inner().try_into().unwrap(), len})
        }
    }
}
impl TryFrom<Bsize> for B32{
    type Error = String;
    fn try_from(value: Bsize) -> Result<Self, Self::Error> {
        let len=value.len();
        if len<Self::MAX_LENGTH{
            Err(format!("B32 only accepts flag lists less than 32 bits long, input had {len} bits"))
        } else {
            Ok(Self { inner: value.as_inner().try_into().unwrap(), len})
        }
    }
}
impl TryFrom<Blong> for B32{
    type Error = String;
    fn try_from(value: Blong) -> Result<Self, Self::Error> {
        let len=value.len();
        if len<Self::MAX_LENGTH{
            Err(format!("B32 only accepts flag lists less than 32 bits long, input had {len} bits"))
        } else {
            Ok(Self { inner: (*value.as_inner().first().unwrap_or(&0)).try_into().unwrap(), len})
        }
    }
}