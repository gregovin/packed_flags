use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, Shl, ShlAssign,
    Shr, ShrAssign, Sub,
};

use crate::{flag_iter, Blong, Bsize, FlagLs, B128, B32};

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug, Hash)]
/// A list of flags up to 64 flags long, or a 64 bit bitfield
pub struct B64 {
    inner: u64,
    len: usize,
}
impl B64 {
    fn lower_mask(point: usize) -> u64 {
        if point > 64 {
            panic!("Cannot create mask more than 64 bits for B64")
        } else {
            (1 << point) - 1
        }
    }
    fn inner(&self) -> u64 {
        self.inner
    }
    #[must_use]
    /// Converts the bitfield into its inner representation, a u64, consuming it
    pub fn as_inner(self) -> u64 {
        self.inner
    }
    fn uper_mask(point: usize) -> u64 {
        if point > 64 {
            panic!("Cannot mask above the end of the list");
        } else {
            u64::MAX - (1 << point) + 1
        }
    }
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    fn init(inner: u64, len: usize) -> Self {
        Self { inner, len }
    }
}
impl Index<usize> for B64 {
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
impl FlagLs for B64 {
    const MAX_LENGTH: usize = 64;

    fn len(&self) -> usize {
        self.len
    }

    fn set_len(&mut self, new_len: usize) {
        assert!(new_len<=Self::MAX_LENGTH,"Cannot set length to a length larger than 64 for B64");
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
            self.inner = (uper << 1) + (u64::from(flag) << index) + lower;
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
            self.inner = uper + (u64::from(flag) << index) + lower;
        } else {
            panic!("Cannot set out of bounds")
        }
    }

    fn iter(&self) -> flag_iter::Iter<Self> {
        flag_iter::Iter::new(self)
    }
}
impl BitAnd<Self> for B64 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::init(self.inner() & rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitAndAssign<Self> for B64 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner &= rhs.inner();
        self.len = self.len.max(rhs.len());
    }
}
impl BitOr<Self> for B64 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::init(self.inner() | rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitOrAssign<Self> for B64 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner |= rhs.inner();
        self.len = self.len.max(rhs.len());
    }
}
impl BitXor<Self> for B64 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::init(self.inner().bitxor(rhs.inner()), self.len().max(rhs.len()))
    }
}
impl BitXorAssign<Self> for B64 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner());
        self.len = self.len.max(rhs.len());
    }
}
impl Shl<usize> for B64 {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        Self::init(self.inner << rhs, (self.len + rhs).min(Self::MAX_LENGTH))
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl ShlAssign<usize> for B64 {
    fn shl_assign(&mut self, rhs: usize) {
        self.inner.shl_assign(rhs);
        self.len = (self.len + rhs).min(Self::MAX_LENGTH);
    }
}
impl Shr<usize> for B64 {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output {
        let new_len = if self.len > rhs { self.len - rhs } else { 0 };
        Self::init(self.inner >> rhs, new_len)
    }
}
impl ShrAssign<usize> for B64 {
    fn shr_assign(&mut self, rhs: usize) {
        self.inner.shr_assign(rhs);
        self.len = if self.len > rhs { self.len - rhs } else { 0 };
    }
}
impl Not for B64 {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self::init((!self.inner) & Self::lower_mask(self.len), self.len)
    }
}
impl Sub<Self> for B64 {
    type Output = Self;
    ///note subtraction is set difference
    fn sub(self, rhs: Self) -> Self::Output {
        Self::init(self.inner & (!rhs.inner), self.len)
    }
}
impl  From<B32> for B64 {
    fn from(value: B32) -> Self {
        let len=value.len();
        Self { inner: value.as_inner().into(), len}
    }
}
impl TryFrom<B128> for B64 {
    type Error = String;
    fn try_from(value: B128) -> Result<Self, Self::Error> {
        let len=value.len();
        if len<Self::MAX_LENGTH{
            Err(format!("B64 only accepts flag lists less than 64 bits long, input had {len} bits"))
        } else {
            Ok(Self { inner: value.as_inner().try_into().unwrap(), len})
        }
    }
}
impl From<Bsize> for B64{
    fn from(value: Bsize) -> Self {
        let len= value.len();
        Self { inner: value.as_inner().try_into().unwrap(), len}
    }
}
impl TryFrom<Blong> for B64{
    type Error = String;
    fn try_from(value: Blong) -> Result<Self, Self::Error> {
        let len=value.len();
        if len<Self::MAX_LENGTH{
            Err(format!("B64 only accepts flag lists less than 64 bits long, input had {len} bits"))
        } else {
            let v_inner=value.as_inner();
            let mut inner=0;
            for (i,item) in v_inner.iter().enumerate(){
                inner+=u64::try_from(*item).unwrap() * (1_u64.checked_shl(usize::BITS * (u32::try_from(i).unwrap())).unwrap_or(0));
            }
            Ok(Self { inner , len})
        }
    }
}